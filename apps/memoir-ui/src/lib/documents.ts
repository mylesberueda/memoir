/**
 * Shared document constants for file upload validation.
 * Used by FileUpload component and the Files page.
 */

export const ACCEPTED_MIME_TYPES = [
	// Images (for OCR and visual content)
	'image/*',
	// Documents
	'application/pdf',
	'text/*',
	'application/rtf',
	// Office formats
	'application/vnd.openxmlformats-officedocument.wordprocessingml.document', // .docx
	'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet', // .xlsx
	'application/vnd.openxmlformats-officedocument.presentationml.presentation', // .pptx
	'application/msword', // .doc
	'application/vnd.ms-excel', // .xls
	'application/vnd.ms-powerpoint', // .ppt
	// Data formats
	'application/json',
	'text/csv',
	// Ebook / markup
	'application/epub+zip',
	'application/xhtml+xml',
	'application/xml',
	'text/html',
];

export const MAX_FILE_SIZE_MB = 10;
export const MAX_FILES_PER_UPLOAD = 5;
