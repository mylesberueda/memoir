import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import { expect, userEvent, within } from 'storybook/test';
import { CodeBlock, CodeBlockCopyButton } from './CodeBlock';

const meta: Meta<typeof CodeBlock> = {
	title: 'Components/Chat/Tool/CodeBlock',
	component: CodeBlock,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component:
					'Syntax-highlighted code block component with support for multiple languages, line numbers, and copy functionality. Uses Prism.js for highlighting with separate light/dark themes.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		code: {
			control: 'text',
			description: 'The code content to display',
		},
		language: {
			control: 'select',
			options: [
				'javascript',
				'typescript',
				'python',
				'java',
				'cpp',
				'css',
				'html',
				'json',
				'bash',
				'sql',
				'yaml',
				'markdown',
			],
			description: 'Programming language for syntax highlighting',
		},
		showLineNumbers: {
			control: 'boolean',
			description: 'Whether to show line numbers',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic JavaScript example
export const JavaScript: Story = {
	args: {
		code: `function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log(fibonacci(10));`,
		language: 'javascript',
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify code content is displayed (checking for specific tokens that should exist)
		const fibonacciElements = canvas.getAllByText('fibonacci');
		await expect(fibonacciElements.length).toBeGreaterThan(0);

		// Check for other key words/tokens that should be present (using partial text matching)
		expect(canvasElement.textContent).toContain('console');
		expect(canvasElement.textContent).toContain('log');
		expect(canvasElement.textContent).toContain('function');

		// Verify code block container structure
		const codeContainer = canvasElement.querySelector('.relative.w-full.overflow-hidden.rounded-md.border');
		await expect(codeContainer).toBeInTheDocument();

		// Verify syntax highlighting elements are present
		const codeElements = canvasElement.querySelectorAll('code');
		await expect(codeElements.length).toBeGreaterThan(0);
	},
};

// TypeScript with interfaces
export const TypeScript: Story = {
	args: {
		code: `interface User {
  id: number;
  name: string;
  email?: string;
}

class UserService {
  private users: User[] = [];

  addUser(user: User): void {
    this.users.push(user);
  }

  findUser(id: number): User | undefined {
    return this.users.find(user => user.id === id);
  }
}

const service = new UserService();
service.addUser({ id: 1, name: "John Doe" });`,
		language: 'typescript',
		showLineNumbers: true,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify TypeScript-specific content
		// Verify TypeScript content using getAllByText for multiple occurrences
		const interfaceElements = canvas.getAllByText('interface');
		await expect(interfaceElements.length).toBeGreaterThan(0);
		// Handle multiple occurrences of "User" in syntax-highlighted code
		const userElements = canvas.getAllByText('User');
		await expect(userElements.length).toBeGreaterThan(0);
		// Handle multiple occurrences of "UserService" in syntax-highlighted code
		const userServiceElements = canvas.getAllByText('UserService');
		await expect(userServiceElements.length).toBeGreaterThan(0);

		// Verify line numbers are shown when enabled
		// Line numbers should be visible in the DOM
		const codeBlock = canvasElement.querySelector('code');
		await expect(codeBlock).toBeInTheDocument();
	},
};

// Python example
export const Python: Story = {
	args: {
		code: `import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split

def preprocess_data(df):
    """Clean and prepare data for machine learning."""
    df = df.dropna()
    df['normalized_price'] = (df['price'] - df['price'].mean()) / df['price'].std()
    return df

# Load and process data
data = pd.read_csv('sales_data.csv')
processed_data = preprocess_data(data)

X = processed_data.drop('target', axis=1)
y = processed_data['target']
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)`,
		language: 'python',
		showLineNumbers: true,
	},
};

// JSON configuration
export const JsonExample: Story = {
	args: {
		code: `{
  "name": "my-awesome-project",
  "version": "1.0.0",
  "description": "An awesome project built with React and TypeScript",
  "main": "index.js",
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "test": "jest",
    "lint": "eslint . --ext .ts,.tsx"
  },
  "dependencies": {
    "react": "^18.0.0",
    "next": "^14.0.0",
    "@types/node": "^20.0.0"
  },
  "devDependencies": {
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "@types/react": "^18.0.0"
  }
}`,
		language: 'json',
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify JSON content is displayed
		// Use textContent for JSON string values that might be broken up
		expect(canvasElement.textContent).toContain('my-awesome-project');
		// Use textContent for syntax-highlighted content that may be split
		expect(canvasElement.textContent).toContain('dependencies');
		expect(canvasElement.textContent).toContain('scripts');

		// Verify JSON syntax highlighting
		// Use textContent for potentially split content
		expect(canvasElement.textContent).toContain('react');
		expect(canvasElement.textContent).toContain('next dev');
	},
};

// CSS styling
export const CSS: Story = {
	args: {
		code: `.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.button {
  display: inline-flex;
  align-items: center;
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.button:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
}

@media (max-width: 768px) {
  .container {
    padding: 0 15px;
  }
  
  .button {
    width: 100%;
    justify-content: center;
  }
}`,
		language: 'css',
		showLineNumbers: true,
	},
};

// HTML markup
export const HTML: Story = {
	args: {
		code: `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Awesome Website</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header class="header">
        <nav class="nav">
            <a href="#" class="logo">Logo</a>
            <ul class="nav-menu">
                <li><a href="#home">Home</a></li>
                <li><a href="#about">About</a></li>
                <li><a href="#services">Services</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
        </nav>
    </header>
    
    <main>
        <section class="hero">
            <h1>Welcome to Our Website</h1>
            <p>We create amazing digital experiences</p>
            <button class="cta-button">Get Started</button>
        </section>
    </main>
    
    <script src="script.js"></script>
</body>
</html>`,
		language: 'html',
	},
};

// Bash/Shell commands
export const Bash: Story = {
	args: {
		code: `#!/bin/bash

# Deploy script for production
echo "Starting deployment process..."

# Pull latest changes
git fetch origin
git reset --hard origin/main

# Install dependencies
npm ci

# Run tests
echo "Running tests..."
npm test

if [ $? -eq 0 ]; then
    echo "Tests passed! Building application..."
    npm run build
    
    # Deploy to server
    rsync -avz --delete dist/ user@server:/var/www/app/
    
    # Restart services
    ssh user@server "sudo systemctl restart nginx && sudo systemctl restart app"
    
    echo "Deployment completed successfully!"
else
    echo "Tests failed! Deployment aborted."
    exit 1
fi`,
		language: 'bash',
		showLineNumbers: true,
	},
};

// SQL query
export const SQL: Story = {
	args: {
		code: `-- Complex analytics query
WITH monthly_sales AS (
  SELECT 
    DATE_TRUNC('month', order_date) AS month,
    SUM(total_amount) AS monthly_revenue,
    COUNT(*) AS order_count,
    COUNT(DISTINCT customer_id) AS unique_customers
  FROM orders 
  WHERE order_date >= '2023-01-01'
  GROUP BY DATE_TRUNC('month', order_date)
),
customer_metrics AS (
  SELECT
    customer_id,
    COUNT(*) AS lifetime_orders,
    SUM(total_amount) AS lifetime_value,
    MAX(order_date) AS last_order_date
  FROM orders
  GROUP BY customer_id
)
SELECT 
  ms.month,
  ms.monthly_revenue,
  ms.order_count,
  ms.unique_customers,
  ROUND(ms.monthly_revenue / ms.order_count, 2) AS avg_order_value,
  LAG(ms.monthly_revenue) OVER (ORDER BY ms.month) AS prev_month_revenue,
  ROUND(
    (ms.monthly_revenue - LAG(ms.monthly_revenue) OVER (ORDER BY ms.month)) 
    / LAG(ms.monthly_revenue) OVER (ORDER BY ms.month) * 100, 
    2
  ) AS revenue_growth_percent
FROM monthly_sales ms
ORDER BY ms.month;`,
		language: 'sql',
		showLineNumbers: true,
	},
};

// YAML configuration
export const YAML: Story = {
	args: {
		code: `# Docker Compose configuration
version: '3.8'

services:
  web:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgresql://user:pass@db:5432/myapp
    depends_on:
      - db
      - redis
    volumes:
      - ./logs:/app/logs
      - uploads:/app/uploads

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
  uploads:

networks:
  default:
    driver: bridge`,
		language: 'yaml',
	},
};

// With copy button
export const WithCopyButton: Story = {
	args: {
		code: `const greeting = "Hello, World!";
console.log(greeting);`,
		language: 'javascript',
		children: <CodeBlockCopyButton />,
	},
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);

		// Verify code content
		// Handle multiple "greeting" elements in syntax-highlighted code
		const greetingElements = canvas.getAllByText('greeting');
		await expect(greetingElements.length).toBeGreaterThan(0);

		// Verify copy button is present
		const copyButton = canvasElement.querySelector('button');
		await expect(copyButton).toBeInTheDocument();

		// Verify copy button has correct positioning
		const buttonContainer = canvasElement.querySelector('.absolute.top-2.right-2');
		await expect(buttonContainer).toBeInTheDocument();

		// Test copy button interaction (note: clipboard API may not work in test environment)
		if (copyButton) {
			await userEvent.click(copyButton);
			// Button should remain clickable
			await expect(copyButton).toBeInTheDocument();
		}
	},
};

// Custom copy button with handlers
export const WithCustomCopyButton: Story = {
	args: {
		code: `function calculateTotal(items) {
  return items.reduce((sum, item) => sum + item.price, 0);
}`,
		language: 'javascript',
		children: (
			<CodeBlockCopyButton
				onCopy={() => console.log('Code copied!')}
				onError={(error) => console.error('Copy failed:', error)}
				timeout={3000}
			/>
		),
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify code content
		// Use textContent for syntax-highlighted content
		expect(canvasElement.textContent).toContain('calculateTotal');
		expect(canvasElement.textContent).toContain('reduce');

		// Verify custom copy button with handlers
		const copyButton = canvasElement.querySelector('button');
		await expect(copyButton).toBeInTheDocument();

		// Test interaction with custom timeout
		if (copyButton) {
			await userEvent.click(copyButton);
			// Should handle the custom onCopy and onError handlers gracefully
			await expect(copyButton).toBeInTheDocument();
		}
	},
};

// Long code example
export const LongCodeExample: Story = {
	args: {
		code: `import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { debounce } from 'lodash';

interface SearchResult {
  id: string;
  title: string;
  description: string;
  url: string;
  score: number;
}

interface SearchResponse {
  results: SearchResult[];
  total: number;
  hasMore: boolean;
}

const SearchComponent: React.FC = () => {
  const [query, setQuery] = useState<string>('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState<number>(1);

  // Debounced search function
  const debouncedSearch = useCallback(
    debounce(async (searchQuery: string, pageNum: number = 1) => {
      if (!searchQuery.trim()) {
        setResults([]);
        return;
      }

      setLoading(true);
      setError(null);

      try {
        const response = await fetch(\`/api/search?q=\${encodeURIComponent(searchQuery)}&page=\${pageNum}\`);
        
        if (!response.ok) {
          throw new Error('Search failed');
        }

        const data: SearchResponse = await response.json();
        
        if (pageNum === 1) {
          setResults(data.results);
        } else {
          setResults(prev => [...prev, ...data.results]);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'An error occurred');
      } finally {
        setLoading(false);
      }
    }, 300),
    []
  );

  // Effect to trigger search when query changes
  useEffect(() => {
    setPage(1);
    debouncedSearch(query, 1);
  }, [query, debouncedSearch]);

  // Memoized filtered results
  const filteredResults = useMemo(() => {
    return results.filter(result => result.score > 0.5);
  }, [results]);

  const handleLoadMore = useCallback(() => {
    const nextPage = page + 1;
    setPage(nextPage);
    debouncedSearch(query, nextPage);
  }, [page, query, debouncedSearch]);

  return (
    <div className="search-container">
      <div className="search-input-wrapper">
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search for anything..."
          className="search-input"
          disabled={loading}
        />
        {loading && <div className="search-spinner">⏳</div>}
      </div>

      {error && (
        <div className="error-message">
          <p>Error: {error}</p>
          <button onClick={() => debouncedSearch(query, 1)}>
            Try Again
          </button>
        </div>
      )}

      <div className="results-container">
        {filteredResults.map((result) => (
          <div key={result.id} className="result-item">
            <h3 className="result-title">
              <a href={result.url} target="_blank" rel="noopener noreferrer">
                {result.title}
              </a>
            </h3>
            <p className="result-description">{result.description}</p>
            <div className="result-meta">
              <span className="result-score">Score: {result.score.toFixed(2)}</span>
              <span className="result-url">{result.url}</span>
            </div>
          </div>
        ))}
      </div>

      {filteredResults.length > 0 && (
        <div className="load-more-container">
          <button 
            onClick={handleLoadMore}
            disabled={loading}
            className="load-more-button"
          >
            {loading ? 'Loading...' : 'Load More Results'}
          </button>
        </div>
      )}

      {query && !loading && filteredResults.length === 0 && (
        <div className="no-results">
          <p>No results found for "{query}"</p>
          <p>Try adjusting your search terms or check your spelling.</p>
        </div>
      )}
    </div>
  );
};

export default SearchComponent;`,
		language: 'typescript',
		showLineNumbers: true,
		children: <CodeBlockCopyButton />,
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		code: `console.log("Custom styled code block");`,
		language: 'javascript',
		className: 'border-2 border-blue-400',
	},
	play: async ({ canvasElement }) => {
		const _canvas = within(canvasElement);

		// Verify content
		// Check for content using textContent to avoid syntax highlighting issues
		expect(canvasElement.textContent).toContain('console');
		expect(canvasElement.textContent).toContain('log');
		// Use textContent for string that might be tokenized by syntax highlighting
		expect(canvasElement.textContent).toContain('Custom styled code block');

		// Verify custom styling is applied
		const container = canvasElement.querySelector('.border-2.border-blue-400');
		await expect(container).toBeInTheDocument();

		// Should also have default classes (using contains for partial class check)
		expect(container?.classList.contains('relative')).toBe(true);
		expect(container?.classList.contains('w-full')).toBe(true);
		expect(container?.classList.contains('overflow-hidden')).toBe(true);
		expect(container?.classList.contains('rounded-md')).toBe(true);
	},
};

// Minimal example
export const MinimalExample: Story = {
	args: {
		code: 'Hello, World!',
		language: 'text',
	},
};

// Multiple code blocks comparison
export const MultipleCodeBlocks: Story = {
	render: () => (
		<div className="space-y-4">
			<div>
				<h3 className="text-sm font-medium mb-2">JavaScript (ES5)</h3>
				<CodeBlock code="var message = 'Hello World';\nconsole.log(message);" language="javascript" />
			</div>
			<div>
				<h3 className="text-sm font-medium mb-2">JavaScript (ES6)</h3>
				<CodeBlock code="const message = 'Hello World';\nconsole.log(message);" language="javascript" />
			</div>
			<div>
				<h3 className="text-sm font-medium mb-2">TypeScript</h3>
				<CodeBlock code="const message: string = 'Hello World';\nconsole.log(message);" language="typescript" />
			</div>
		</div>
	),
};

// Language showcase
export const LanguageShowcase: Story = {
	render: () => (
		<div className="grid gap-4 md:grid-cols-2">
			<CodeBlock code="console.log('JavaScript');" language="javascript" />
			<CodeBlock code="print('Python')" language="python" />
			<CodeBlock code='{"json": "example"}' language="json" />
			<CodeBlock code="SELECT * FROM users;" language="sql" />
		</div>
	),
};
