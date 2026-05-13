import axios, { type AxiosInstance } from 'axios';

export interface MaildevEmail {
	id: string;
	to: Array<{ address: string; name: string }>;
	from: Array<{ address: string; name: string }>;
	subject: string;
	html: string;
	text: string;
	date: string;
}

export class MaildevClient {
	private client: AxiosInstance;

	constructor(baseUrl = 'http://localhost:1080') {
		this.client = axios.create({
			baseURL: baseUrl,
			timeout: 8000,
		});
	}

	async getAllEmails(): Promise<MaildevEmail[]> {
		const response = await this.client.get<MaildevEmail[]>('/email');
		return response.data;
	}

	async getEmail(id: string): Promise<MaildevEmail> {
		const response = await this.client.get<MaildevEmail>(`/email/${id}`);
		return response.data;
	}

	async getEmailsForAddress(emailAddress: string): Promise<MaildevEmail[]> {
		const emails = await this.getAllEmails();
		return emails.filter((email) => email.to.some((recipient) => recipient.address === emailAddress));
	}

	async waitForEmail(emailAddress: string, timeoutMs = 8000, pollIntervalMs = 500): Promise<MaildevEmail> {
		const startTime = Date.now();

		while (Date.now() - startTime < timeoutMs) {
			const emails = await this.getEmailsForAddress(emailAddress);
			if (emails.length > 0) {
				// Return the most recent email
				return emails.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())[0];
			}
			await new Promise((resolve) => setTimeout(resolve, pollIntervalMs));
		}

		throw new Error(`No email received for ${emailAddress} within ${timeoutMs}ms`);
	}

	async deleteAllEmails(): Promise<void> {
		await this.client.delete('/email');
	}

	extractVerificationUrl(emailHtml: string): string | null {
		// Zitadel verification URL format:
		// http://localhost:5150/ui/login/mail/verification?authRequestID=&code=XXX&orgID=XXX&userID=XXX
		const match = emailHtml.match(/href="([^"]*\/ui\/login\/mail\/verification\?[^"]+)"/);
		return match ? match[1] : null;
	}

	async getMessages(emailAddress: string): Promise<MaildevEmail[]> {
		return this.getEmailsForAddress(emailAddress);
	}
}
