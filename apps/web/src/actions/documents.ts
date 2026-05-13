'use server';

import { create } from '@bufbuild/protobuf';
import { documentGroupServiceClient, documentServiceClient, inferenceServiceClient } from '@lib/grpc/clients';
import { createChildLogger } from '@lib/logger';
import {
	DeleteDocumentRequestSchema,
	type Document,
	type DocumentGroup,
	type DocumentStatus,
	GetDocumentRequestSchema,
	GetDownloadUrlRequestSchema,
	ListDocumentsRequestSchema,
	ListGroupsRequestSchema,
	UploadDocumentRequestSchema,
} from '@startup/proto-ts/rig-service/rig/v1/document_pb';
import {
	AttachDocumentsRequestSchema,
	DetachDocumentsRequestSchema,
	ListConversationDocumentsRequestSchema,
} from '@startup/proto-ts/rig-service/rig/v1/inference_pb';
import type { ActionResult } from '.';

const log = createChildLogger({ action: 'documents' });

export async function uploadDocument(input: {
	filename: string;
	contentType: string;
	content: Uint8Array;
	conversationPid?: string;
}): Promise<ActionResult<{ document: Document | undefined }>> {
	try {
		const client = await documentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(UploadDocumentRequestSchema, {
			filename: input.filename,
			contentType: input.contentType,
			content: input.content,
			conversationPid: input.conversationPid,
		});

		const res = await client.uploadDocument(req);

		log.debug('uploadDocument response', {
			documentPid: res.document?.pid,
		});

		return {
			success: true,
			data: { document: res.document },
		};
	} catch (error) {
		log.error('uploadDocument error', { error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function fetchDocument(pid: string): Promise<ActionResult<{ document: Document | undefined }>> {
	try {
		const client = await documentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetDocumentRequestSchema, { pid });
		const res = await client.getDocument(req);

		return {
			success: true,
			data: { document: res.document },
		};
	} catch (error) {
		log.error('fetchDocument error', { pid, error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function fetchDocuments(options?: {
	groupPid?: string;
	status?: DocumentStatus;
	page?: number;
	pageSize?: number;
}): Promise<ActionResult<{ documents: Document[]; total: number }>> {
	try {
		const client = await documentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListDocumentsRequestSchema, {
			groupPid: options?.groupPid,
			status: options?.status,
			page: options?.page ?? 1,
			pageSize: options?.pageSize ?? 50,
		});

		const res = await client.listDocuments(req);

		log.debug('fetchDocuments response', {
			total: res.total,
			documentCount: res.documents.length,
		});

		return {
			success: true,
			data: {
				documents: res.documents,
				total: res.total,
			},
		};
	} catch (error) {
		log.error('fetchDocuments error', { error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function deleteDocument(pid: string): Promise<{ success: true } | { success: false; error: string }> {
	try {
		const client = await documentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DeleteDocumentRequestSchema, { pid });
		await client.deleteDocument(req);

		return { success: true };
	} catch (error) {
		log.error('deleteDocument error', { pid, error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function getDownloadUrl(pid: string): Promise<ActionResult<{ downloadUrl: string }>> {
	try {
		const client = await documentServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(GetDownloadUrlRequestSchema, { pid });
		const res = await client.getDownloadUrl(req);

		return {
			success: true,
			data: { downloadUrl: res.downloadUrl },
		};
	} catch (error) {
		log.error('getDownloadUrl error', { pid, error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

// ─── InferenceService conversation-document RPCs ─────────────────────────────

export async function fetchConversationDocuments(
	conversationPid: string,
): Promise<ActionResult<{ documents: Document[] }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListConversationDocumentsRequestSchema, { conversationPid });
		const res = await client.listConversationDocuments(req);

		log.debug('fetchConversationDocuments response', {
			conversationPid,
			documentCount: res.documents.length,
		});

		return {
			success: true,
			data: { documents: res.documents },
		};
	} catch (error) {
		log.error('fetchConversationDocuments error', {
			conversationPid,
			error: error instanceof Error ? error.message : error,
		});
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function attachDocuments(
	conversationPid: string,
	documentPids: string[],
): Promise<ActionResult<{ attachedCount: number }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(AttachDocumentsRequestSchema, {
			conversationPid,
			documentPids,
		});
		const res = await client.attachDocuments(req);

		return {
			success: true,
			data: { attachedCount: res.attachedCount },
		};
	} catch (error) {
		log.error('attachDocuments error', {
			conversationPid,
			error: error instanceof Error ? error.message : error,
		});
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

export async function detachDocuments(
	conversationPid: string,
	documentPids: string[],
): Promise<ActionResult<{ detachedCount: number }>> {
	try {
		const client = await inferenceServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(DetachDocumentsRequestSchema, {
			conversationPid,
			documentPids,
		});
		const res = await client.detachDocuments(req);

		return {
			success: true,
			data: { detachedCount: res.detachedCount },
		};
	} catch (error) {
		log.error('detachDocuments error', {
			conversationPid,
			error: error instanceof Error ? error.message : error,
		});
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}

// ─── DocumentGroupService RPCs ───────────────────────────────────────────────

export async function fetchGroups(options?: {
	page?: number;
	pageSize?: number;
}): Promise<ActionResult<{ groups: DocumentGroup[]; total: number }>> {
	try {
		const client = await documentGroupServiceClient();
		if (!client) {
			return { success: false, error: 'Authentication required' };
		}

		const req = create(ListGroupsRequestSchema, {
			page: options?.page ?? 1,
			pageSize: options?.pageSize ?? 50,
		});

		const res = await client.listGroups(req);

		return {
			success: true,
			data: {
				groups: res.groups,
				total: res.total,
			},
		};
	} catch (error) {
		log.error('fetchGroups error', { error: error instanceof Error ? error.message : error });
		return {
			success: false,
			error: error instanceof Error ? error.message : 'An unexpected error occurred',
		};
	}
}
