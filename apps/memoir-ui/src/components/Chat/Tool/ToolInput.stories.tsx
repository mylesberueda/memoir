import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, within } from 'storybook/test';
import ToolInput from './ToolInput';

const meta: Meta<typeof ToolInput> = {
	title: 'Components/Chat/Tool/ToolInput',
	component: ToolInput,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component: 'Displays tool input parameters in a formatted JSON code block with syntax highlighting.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		input: {
			control: 'object',
			description: 'The input parameters object to display',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Simple string parameter
export const SimpleString: Story = {
	args: {
		input: {
			query: 'React hooks tutorial',
		},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify parameters header
		await expect(canvas.getByText('Parameters')).toBeInTheDocument();

		// Verify JSON content is displayed using textContent for syntax-highlighted strings
		expect(canvasElement.textContent).toContain('React hooks tutorial');

		// Verify code block structure
		// Find element with bg-muted/50 class using class list check
		const codeBlocks = canvasElement.querySelectorAll('div');
		const codeBlock = Array.from(codeBlocks).find((div) => div.classList.contains('bg-muted/50'));
		await expect(codeBlock).toBeDefined();
	},
};

// Multiple parameters
export const MultipleParameters: Story = {
	args: {
		input: {
			query: 'JavaScript best practices',
			maxResults: 10,
			language: 'en',
			includeSummary: true,
		},
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify all parameter values are displayed using textContent for syntax-highlighted values
		expect(canvasElement.textContent).toContain('JavaScript best practices');
		expect(canvasElement.textContent).toContain('10');
		expect(canvasElement.textContent).toContain('en');
		expect(canvasElement.textContent).toContain('true');

		// Verify JSON formatting using textContent for syntax-highlighted keys
		expect(canvasElement.textContent).toContain('query');
		expect(canvasElement.textContent).toContain('maxResults');
	},
};

// Nested object parameters
export const NestedObject: Story = {
	args: {
		input: {
			searchCriteria: {
				keywords: ['react', 'typescript', 'hooks'],
				filters: {
					dateRange: '2023-01-01 to 2023-12-31',
					category: 'development',
					difficulty: 'intermediate',
				},
			},
			options: {
				maxResults: 50,
				sortBy: 'relevance',
				includeThumbnails: false,
			},
		},
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify nested object structure using textContent for syntax-highlighted keys
		expect(canvasElement.textContent).toContain('searchCriteria');
		expect(canvasElement.textContent).toContain('keywords');
		expect(canvasElement.textContent).toContain('filters');
		expect(canvasElement.textContent).toContain('options');

		// Verify array and nested values using textContent
		expect(canvasElement.textContent).toContain('react');
		expect(canvasElement.textContent).toContain('development');
		expect(canvasElement.textContent).toContain('relevance');
	},
};

// Array parameters
export const ArrayParameters: Story = {
	args: {
		input: {
			files: ['index.js', 'components.jsx', 'utils.ts'],
			extensions: ['.js', '.jsx', '.ts', '.tsx'],
			exclude: ['node_modules', '.git', 'dist'],
		},
	},
};

// Complex data structure
export const ComplexDataStructure: Story = {
	args: {
		input: {
			dataset: 'sales_data.csv',
			analysis: {
				type: 'correlation',
				columns: ['revenue', 'marketing_spend', 'customer_count'],
				aggregations: [
					{ column: 'revenue', function: 'sum' },
					{ column: 'customer_count', function: 'avg' },
				],
			},
			filters: {
				dateRange: {
					start: '2023-01-01',
					end: '2023-12-31',
				},
				regions: ['North America', 'Europe'],
				minimumRevenue: 1000,
			},
			output: {
				format: 'json',
				includeMetadata: true,
				precision: 2,
			},
		},
	},
};

// Code execution parameters
export const CodeExecution: Story = {
	args: {
		input: {
			language: 'python',
			code: `
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print([fibonacci(i) for i in range(10)])
			`.trim(),
			timeout: 30000,
			dependencies: ['numpy', 'pandas'],
		},
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify code content is displayed using textContent
		expect(canvasElement.textContent).toContain('fibonacci');
		expect(canvasElement.textContent).toContain('python');

		// Verify dependencies array using textContent
		expect(canvasElement.textContent).toContain('numpy');
		expect(canvasElement.textContent).toContain('pandas');

		// Verify timeout value using textContent
		expect(canvasElement.textContent).toContain('30000');
	},
};

// File operation parameters
export const FileOperation: Story = {
	args: {
		input: {
			operation: 'read',
			path: '/home/user/documents/report.pdf',
			encoding: 'utf-8',
			options: {
				parseMetadata: true,
				extractText: true,
				includeThumbnail: false,
			},
		},
	},
};

// API call parameters
export const APICall: Story = {
	args: {
		input: {
			method: 'POST',
			url: 'https://api.example.com/v1/analysis',
			headers: {
				'Content-Type': 'application/json',
				Authorization: 'Bearer [REDACTED]',
				'User-Agent': 'AI-Assistant/1.0',
			},
			body: {
				text: 'Analyze this customer feedback',
				options: {
					sentiment: true,
					keywords: true,
					categories: ['product', 'service', 'pricing'],
				},
			},
			timeout: 10000,
		},
	},
};

// Empty input
export const EmptyInput: Story = {
	args: {
		input: {},
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify parameters header still appears
		await expect(canvas.getByText('Parameters')).toBeInTheDocument();

		// Verify empty object is displayed (JSON is syntax-highlighted)
		expect(canvasElement.textContent).toContain('{}');
	},
};

// Boolean and null values
export const MixedDataTypes: Story = {
	args: {
		input: {
			enabled: true,
			disabled: false,
			optionalField: null,
			count: 42,
			percentage: 85.7,
			tags: [],
			metadata: {},
		},
	},
};

// Long text content
export const LongTextContent: Story = {
	args: {
		input: {
			prompt:
				'Write a comprehensive analysis of modern web development frameworks, including their strengths, weaknesses, and use cases. Cover React, Vue, Angular, Svelte, and newer frameworks like Solid.js and Qwik.',
			maxLength: 5000,
			style: 'technical',
			includeCodeExamples: true,
		},
	},
};

// Database query parameters
export const DatabaseQuery: Story = {
	args: {
		input: {
			query:
				'SELECT u.name, COUNT(o.id) as order_count FROM users u LEFT JOIN orders o ON u.id = o.user_id WHERE u.created_at >= ? GROUP BY u.id ORDER BY order_count DESC LIMIT ?',
			parameters: ['2023-01-01', 100],
			database: 'production',
			options: {
				timeout: 30000,
				readOnly: true,
				explain: false,
			},
		},
	},
};

// Machine learning parameters
export const MachineLearning: Story = {
	args: {
		input: {
			model: 'text-classification',
			algorithm: 'transformer',
			hyperparameters: {
				learning_rate: 0.001,
				batch_size: 32,
				epochs: 10,
				dropout: 0.1,
			},
			data: {
				train_split: 0.8,
				validation_split: 0.1,
				test_split: 0.1,
				shuffle: true,
				random_state: 42,
			},
			features: ['text_content', 'word_count', 'sentiment_score'],
			target: 'category',
		},
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		input: {
			message: 'This input has custom styling',
		},
		className: 'border-2 border-primary',
	},
};

// Very large object
export const VeryLargeObject: Story = {
	args: {
		input: {
			config: {
				server: {
					host: 'localhost',
					port: 3000,
					ssl: {
						enabled: true,
						cert: '/path/to/cert.pem',
						key: '/path/to/key.pem',
					},
				},
				database: {
					type: 'postgresql',
					host: 'db.example.com',
					port: 5432,
					name: 'app_db',
					pool: {
						min: 2,
						max: 10,
						idle: 10000,
					},
					logging: false,
				},
				cache: {
					type: 'redis',
					host: 'cache.example.com',
					port: 6379,
					ttl: 3600,
					prefix: 'app:',
				},
				features: {
					authentication: true,
					rateLimiting: true,
					logging: true,
					metrics: true,
					caching: true,
				},
				integrations: {
					stripe: {
						apiKey: '[REDACTED]',
						webhookSecret: '[REDACTED]',
					},
					sendgrid: {
						apiKey: '[REDACTED]',
						fromEmail: 'noreply@example.com',
					},
				},
			},
		},
	},
};
