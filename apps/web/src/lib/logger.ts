/* v8 ignore start */
import 'server-only';

import winston from 'winston';

const { combine, timestamp, json, errors, colorize, printf } = winston.format;

/**
 * Determine environment.
 * Check for VITEST env var (set by Vitest) in addition to NODE_ENV
 * because vitest.config.ts may override process.env.
 */
const isTest = process.env.NODE_ENV === 'test' || process.env.VITEST === 'true';
const isDev = process.env.NODE_ENV === 'development';

/**
 * Custom format for development - more readable than JSON.
 */
const devFormat = printf(({ level, message, timestamp, service, ...meta }) => {
	const metaStr = Object.keys(meta).length ? ` ${JSON.stringify(meta)}` : '';
	const serviceStr = service ? `[${service}] ` : '';
	return `${timestamp} ${level}: ${serviceStr}${message}${metaStr}`;
});

/**
 * Create the base logger configuration.
 */
function createLogger() {
	const transports: winston.transport[] = [];

	if (isTest) {
		// Silent in tests - use vi.spyOn(logger, 'error') to test logging
		transports.push(
			new winston.transports.Console({
				silent: true,
			}),
		);
	} else if (isDev) {
		// Pretty output for development
		transports.push(
			new winston.transports.Console({
				format: combine(colorize(), timestamp({ format: 'HH:mm:ss' }), devFormat),
			}),
		);
	} else {
		// JSON output for production (structured logging)
		transports.push(
			new winston.transports.Console({
				format: combine(timestamp(), errors({ stack: true }), json()),
			}),
		);
	}

	return winston.createLogger({
		level: process.env.LOG_LEVEL || (isDev ? 'debug' : 'info'),
		defaultMeta: { service: 'web' },
		transports,
	});
}

/**
 * The root logger instance.
 */
export const logger = createLogger();

/**
 * Create a child logger with additional context.
 * Use this to add context like action name, request ID, etc.
 *
 * @example
 * const log = createChildLogger({ action: 'createOrg' });
 * log.info('Creating organization', { name: 'Acme' });
 * log.error('Failed to create organization', { error: err.message });
 */
export function createChildLogger(meta: Record<string, unknown>) {
	return logger.child(meta);
}

/**
 * Pre-configured child loggers for common contexts.
 */
export const actionLogger = createChildLogger({ context: 'action' });
export const authLogger = createChildLogger({ context: 'auth' });
export const grpcLogger = createChildLogger({ context: 'grpc' });
