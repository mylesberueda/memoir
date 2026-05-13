'use server';

import { createHmac, randomBytes, timingSafeEqual } from 'node:crypto';
import Redis from 'ioredis';
import { cookies } from 'next/headers';
import { authLogger } from './logger';

const SESSION_PREFIX = 'web:session:';
const USER_SESSIONS_PREFIX = 'web:user:';
const SESSION_COOKIE_NAME = 'session_token';
const SESSION_TTL_SECONDS = 24 * 60 * 60; // 24 hours

export interface SessionData {
	accessToken: string;
	refreshToken: string;
	idToken: string;
	userId: string;
	expiresAt: number; // Unix timestamp in seconds
	forceRefresh?: boolean; // Set by webhook to trigger token refresh on next request
}

/**
 * Get session secrets from environment.
 * Supports rotation via comma-separated values: "new-secret,old-secret,..."
 * - First secret is used for signing new tokens
 * - All secrets are tried for validation (enables graceful rotation)
 * - To rotate: prepend new secret, remove oldest after 24h TTL passes
 */
function getSessionSecrets(): string[] {
	const secretEnv = process.env.SESSION_SECRET;
	if (!secretEnv) {
		throw new Error('SESSION_SECRET environment variable is not set');
	}

	const secrets = secretEnv
		.split(',')
		.map((s) => s.trim())
		.filter(Boolean);
	if (secrets.length === 0 || secrets[0].length < 32) {
		throw new Error('SESSION_SECRET must have at least one secret of 32+ characters');
	}

	return secrets;
}

function getSigningSecret(): string {
	return getSessionSecrets()[0];
}

function signSessionId(sessionId: string): string {
	const signature = createHmac('sha256', getSigningSecret()).update(sessionId).digest('base64url');
	return `${sessionId}.${signature}`;
}

function verifyAndExtractSessionId(signedToken: string): string | null {
	const parts = signedToken.split('.');
	if (parts.length !== 2) return null;

	const [sessionId, signature] = parts;
	if (!sessionId || !signature) return null;

	const sigBuffer = Buffer.from(signature, 'base64url');

	// Try all secrets for validation (supports rotation)
	for (const secret of getSessionSecrets()) {
		const expectedSignature = createHmac('sha256', secret).update(sessionId).digest('base64url');

		const expectedBuffer = Buffer.from(expectedSignature, 'base64url');

		if (sigBuffer.length === expectedBuffer.length && timingSafeEqual(sigBuffer, expectedBuffer)) {
			return sessionId;
		}
	}

	return null;
}

let redisClient: Redis | null = null;

function getRedisClient(): Redis {
	if (!redisClient) {
		const redisUrl = process.env.REDIS_URL;
		if (!redisUrl) {
			throw new Error('REDIS_URL environment variable is not set');
		}
		redisClient = new Redis(redisUrl, {
			maxRetriesPerRequest: 3,
			lazyConnect: true,
		});
	}
	return redisClient;
}

function generateSessionId(): string {
	return randomBytes(32).toString('hex');
}

function sessionKey(sessionId: string): string {
	return `${SESSION_PREFIX}${sessionId}`;
}

function userSessionsKey(userId: string): string {
	return `${USER_SESSIONS_PREFIX}${userId}:sessions`;
}

export async function createSession(data: SessionData): Promise<string> {
	const redis = getRedisClient();
	const sessionId = generateSessionId();
	const key = sessionKey(sessionId);
	const userKey = userSessionsKey(data.userId);

	// Store session data
	await redis.set(key, JSON.stringify(data), 'EX', SESSION_TTL_SECONDS);

	// Lazy cleanup: remove stale session IDs from user's set
	// This keeps reads fast; cleanup happens on login/refresh (infrequent)
	const existingIds = await redis.smembers(userKey);
	if (existingIds.length > 0) {
		// Batch check which sessions still exist (pipeline for efficiency)
		const pipeline = redis.pipeline();
		for (const id of existingIds) {
			pipeline.exists(sessionKey(id));
		}
		const results = await pipeline.exec();

		// Remove IDs where session no longer exists (TTL expired)
		const staleIds = existingIds.filter((_, i) => results?.[i]?.[1] === 0);
		if (staleIds.length > 0) {
			await redis.srem(userKey, ...staleIds);
		}
	}

	// Add new session to user's set
	await redis.sadd(userKey, sessionId);
	await redis.expire(userKey, SESSION_TTL_SECONDS);

	// Sign the session ID before storing in cookie
	const signedToken = signSessionId(sessionId);

	// Set HTTP-only cookie with signed session token
	const cookieStore = await cookies();
	cookieStore.set(SESSION_COOKIE_NAME, signedToken, {
		httpOnly: true,
		secure: process.env.NODE_ENV === 'production',
		sameSite: 'lax',
		maxAge: SESSION_TTL_SECONDS,
		path: '/',
	});

	return sessionId;
}

export async function getSession(): Promise<SessionData | null> {
	const cookieStore = await cookies();
	const signedToken = cookieStore.get(SESSION_COOKIE_NAME)?.value;

	if (!signedToken) {
		return null;
	}

	// Verify signature and extract session ID
	const sessionId = verifyAndExtractSessionId(signedToken);
	if (!sessionId) {
		// Invalid signature - likely forged or corrupted
		authLogger.warn('Invalid session cookie signature detected');
		return null;
	}

	return getSessionById(sessionId);
}

export async function getSessionById(sessionId: string): Promise<SessionData | null> {
	const redis = getRedisClient();
	const key = sessionKey(sessionId);

	const data = await redis.get(key);
	if (!data) {
		return null;
	}

	try {
		return JSON.parse(data) as SessionData;
	} catch (error) {
		authLogger.error('Failed to parse session data from Redis', { error, sessionId });
		return null;
	}
}

/**
 * Set session data by session ID (internal use by auth providers).
 * Used by AuthProvider.storeSession() to write tokens to Redis.
 *
 * @param sessionId - Session identifier
 * @param data - Session data to store
 */
export async function setSessionById(sessionId: string, data: SessionData): Promise<void> {
	const redis = getRedisClient();
	const key = sessionKey(sessionId);
	const userKey = userSessionsKey(data.userId);

	// Store session data
	await redis.set(key, JSON.stringify(data), 'EX', SESSION_TTL_SECONDS);

	// Add to user's session set
	await redis.sadd(userKey, sessionId);
	await redis.expire(userKey, SESSION_TTL_SECONDS);
}

export async function updateSession(data: Partial<SessionData>): Promise<boolean> {
	const cookieStore = await cookies();
	const signedToken = cookieStore.get(SESSION_COOKIE_NAME)?.value;

	if (!signedToken) {
		return false;
	}

	const sessionId = verifyAndExtractSessionId(signedToken);
	if (!sessionId) {
		return false;
	}

	const existing = await getSessionById(sessionId);
	if (!existing) {
		return false;
	}

	const redis = getRedisClient();
	const key = sessionKey(sessionId);
	const updated = { ...existing, ...data };

	await redis.set(key, JSON.stringify(updated), 'EX', SESSION_TTL_SECONDS);
	return true;
}

export async function deleteSession(): Promise<void> {
	const cookieStore = await cookies();
	const signedToken = cookieStore.get(SESSION_COOKIE_NAME)?.value;

	if (signedToken) {
		const sessionId = verifyAndExtractSessionId(signedToken);
		if (sessionId) {
			const redis = getRedisClient();
			const key = sessionKey(sessionId);

			// Get session data to find userId for cleanup
			const data = await redis.get(key);
			if (data) {
				try {
					const session = JSON.parse(data) as SessionData;
					// Remove from user's session set
					await redis.srem(userSessionsKey(session.userId), sessionId);
				} catch {
					// Ignore parse errors
				}
			}

			await redis.del(key);
		}
	}

	cookieStore.delete(SESSION_COOKIE_NAME);
}

export async function deleteAllUserSessions(userId: string): Promise<number> {
	const redis = getRedisClient();
	const userKey = userSessionsKey(userId);

	// Get all session IDs for this user
	const sessionIds = await redis.smembers(userKey);

	if (sessionIds.length === 0) {
		return 0;
	}

	// Delete all session data
	const sessionKeys = sessionIds.map((id) => sessionKey(id));
	await redis.del(...sessionKeys);

	// Delete the user sessions set
	await redis.del(userKey);

	return sessionIds.length;
}

export async function getUserSessionCount(userId: string): Promise<number> {
	const redis = getRedisClient();
	const userKey = userSessionsKey(userId);
	return redis.scard(userKey);
}

/**
 * Mark all sessions for a user to force token refresh on next request.
 * Called by webhook when user metadata changes (e.g., tier update).
 * Does NOT delete sessions - user stays logged in.
 */
export async function forceRefreshAllUserSessions(userId: string): Promise<number> {
	const redis = getRedisClient();
	const userKey = userSessionsKey(userId);

	// Get all session IDs for this user
	const sessionIds = await redis.smembers(userKey);

	if (sessionIds.length === 0) {
		return 0;
	}

	let updated = 0;

	// Update each session to set forceRefresh flag
	for (const id of sessionIds) {
		const key = sessionKey(id);
		const data = await redis.get(key);

		if (data) {
			try {
				const session = JSON.parse(data) as SessionData;
				session.forceRefresh = true;
				await redis.set(key, JSON.stringify(session), 'EX', SESSION_TTL_SECONDS);
				updated++;
			} catch {
				// Session data corrupted, skip
				authLogger.warn('Failed to parse session data for force refresh', { sessionId: id });
			}
		}
	}

	authLogger.info('Marked sessions for force refresh', { userId, updated, total: sessionIds.length });

	return updated;
}

export async function getSessionId(): Promise<string | null> {
	const cookieStore = await cookies();
	const signedToken = cookieStore.get(SESSION_COOKIE_NAME)?.value;

	if (!signedToken) {
		return null;
	}

	return verifyAndExtractSessionId(signedToken);
}
