import type { Meta, StoryObj } from '@storybook/nextjs-vite';
import Response from './Response';

const meta: Meta<typeof Response> = {
	title: 'Components/Chat/Response',
	component: Response,
	parameters: {
		layout: 'padded',
		docs: {
			description: {
				component: 'Response component for rendering assistant messages with markdown support using Streamdown.',
			},
		},
	},
	tags: ['autodocs'],
	argTypes: {
		children: {
			control: 'text',
			description: 'The markdown content to render',
		},
		className: {
			control: 'text',
			description: 'Additional CSS classes',
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

// Basic text response
export const BasicText: Story = {
	args: {
		children: 'This is a simple text response from the assistant.',
	},
};

// Markdown content
export const MarkdownContent: Story = {
	args: {
		children: `# Heading 1

This is a **bold** text and this is *italic* text.

## Heading 2

Here's a list:
- Item 1
- Item 2
- Item 3

### Code Example

\`\`\`javascript
function greet(name) {
  return \`Hello, \${name}!\`;
}

console.log(greet('World'));
\`\`\`

And here's some inline \`code\`.`,
	},
};

// Multi-line response with line breaks
export const MultiLineResponse: Story = {
	args: {
		children: `Here's a multi-line response that demonstrates how the Response component handles line breaks.

This is the second paragraph with proper spacing.

And this is the third paragraph showing consistent formatting.

Each paragraph maintains proper visual hierarchy and readability.`,
	},
};

// Code-heavy response
export const CodeHeavyResponse: Story = {
	args: {
		children: `Here's how to create a React component:

\`\`\`tsx
import React from 'react';

interface Props {
  title: string;
  description?: string;
}

export const Card: React.FC<Props> = ({ title, description }) => {
  return (
    <div className="card">
      <h2>{title}</h2>
      {description && <p>{description}</p>}
    </div>
  );
};
\`\`\`

You can also use inline code like \`useState\` and \`useEffect\` hooks.

Here's a CSS example:

\`\`\`css
.card {
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 16px;
  margin: 8px;
}
\`\`\``,
	},
};

// List and table response
export const ListsAndTables: Story = {
	args: {
		children: `## Task List

Here's what we need to do:

1. **Set up the project structure**
   - Create components folder
   - Set up routing
   - Configure styling

2. **Implement core features**
   - User authentication
   - Data management
   - API integration

3. **Testing and deployment**
   - Write unit tests
   - E2E testing
   - Deploy to production

## Comparison Table

| Feature | Option A | Option B |
|---------|----------|----------|
| Performance | Fast | Very Fast |
| Ease of Use | Easy | Moderate |
| Cost | Free | $10/month |
| Support | Community | 24/7 |`,
	},
};

// Long response
export const LongResponse: Story = {
	args: {
		children: `# Comprehensive Guide to Modern Web Development

Modern web development has evolved significantly over the past decade. Today's developers have access to powerful tools, frameworks, and methodologies that enable them to build sophisticated, scalable applications.

## Frontend Technologies

### React and the Component Ecosystem

React has revolutionized how we think about user interfaces. Its component-based architecture promotes reusability and maintainability. Here are some key concepts:

- **Components**: Reusable UI building blocks
- **Props**: Data passed between components
- **State**: Component-level data management
- **Hooks**: Modern way to handle state and side effects

\`\`\`jsx
import React, { useState, useEffect } from 'react';

function UserProfile({ userId }) {
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetchUser(userId).then(setUser);
  }, [userId]);

  if (!user) return <div>Loading...</div>;

  return (
    <div className="user-profile">
      <h1>{user.name}</h1>
      <p>{user.email}</p>
    </div>
  );
}
\`\`\`

### Styling Solutions

Modern CSS frameworks and solutions:

1. **Tailwind CSS** - Utility-first framework
2. **Styled Components** - CSS-in-JS solution
3. **CSS Modules** - Scoped styling
4. **SASS/SCSS** - CSS preprocessor

## Backend Development

### API Design

RESTful APIs remain popular, but GraphQL is gaining traction:

- **REST**: Simple, cacheable, stateless
- **GraphQL**: Flexible queries, single endpoint
- **tRPC**: Type-safe APIs for TypeScript

### Database Choices

Different applications require different database solutions:

| Database Type | Use Cases | Examples |
|---------------|-----------|----------|
| Relational | ACID compliance, complex queries | PostgreSQL, MySQL |
| Document | Flexible schemas, rapid development | MongoDB, CouchDB |
| Graph | Connected data, relationships | Neo4j, ArangoDB |
| Key-Value | Caching, session storage | Redis, DynamoDB |

## Deployment and DevOps

Modern deployment strategies focus on automation and reliability:

### Continuous Integration/Continuous Deployment (CI/CD)

1. **Code commits** trigger automated builds
2. **Tests run** automatically on all changes
3. **Deployment** happens seamlessly to production
4. **Monitoring** ensures application health

### Containerization with Docker

\`\`\`dockerfile
FROM node:18-alpine

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]
\`\`\`

This approach ensures consistent environments across development, staging, and production.

## Best Practices

### Code Quality

- **TypeScript** for type safety
- **ESLint** for code linting
- **Prettier** for code formatting
- **Husky** for git hooks

### Testing Strategy

- **Unit tests** for individual functions
- **Integration tests** for component interactions
- **E2E tests** for user workflows
- **Performance tests** for load handling

### Security Considerations

Security should be built in from the start:

1. **Authentication** - Who are you?
2. **Authorization** - What can you do?
3. **Data validation** - Is the input safe?
4. **HTTPS everywhere** - Encrypt all communications
5. **Regular updates** - Keep dependencies current

## Conclusion

Web development continues to evolve rapidly. The key to success is staying curious, continuously learning, and building projects that solve real problems for real users.

Remember: the best technology stack is the one that helps you ship quality products efficiently and maintainably.`,
	},
};

// Response with special formatting
export const SpecialFormatting: Story = {
	args: {
		children: `## Quick Reference

> **Important Note**: This is a blockquote that highlights important information.

### Keyboard Shortcuts

- \`Ctrl+C\` - Copy
- \`Ctrl+V\` - Paste
- \`Ctrl+Z\` - Undo
- \`Ctrl+Y\` - Redo

### Links and References

Check out [React Documentation](https://react.dev) for more information.

You can also visit:
- [MDN Web Docs](https://developer.mozilla.org)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

---

*That's all for now! Happy coding!*`,
	},
};

// Response with thinking blocks (parsed out)
export const WithThinkingBlocks: Story = {
	args: {
		children: `Based on my analysis, here's the solution:

\`\`\`javascript
function optimizePerformance() {
  // Use memoization for expensive calculations
  const memoizedResult = useMemo(() => {
    return expensiveOperation();
  }, [dependencies]);

  return memoizedResult;
}
\`\`\`

This approach will significantly improve your component's performance.`,
	},
};

// Response with multiple code blocks
export const MultipleCodeBlocks: Story = {
	args: {
		children: `Let me show you different approaches:

### Approach 1: Using useState

\`\`\`tsx
const [count, setCount] = useState(0);
\`\`\`

### Approach 2: Using useReducer

\`\`\`tsx
const [state, dispatch] = useReducer(reducer, initialState);
\`\`\`

### Approach 3: Using external state

\`\`\`tsx
const count = useStore(state => state.count);
\`\`\`

Each approach has its own benefits depending on your use case.`,
	},
};

// Response with inline code and emphasis
export const InlineCodeAndEmphasis: Story = {
	args: {
		children: `When working with React, remember that \`useState\` is **asynchronous**. This means:

- Calling \`setState\` doesn't immediately update the state
- You need to use the *functional update pattern* for dependent updates
- Always use \`useEffect\` to respond to state changes

For example: \`setState(prev => prev + 1)\` is safer than \`setState(count + 1)\`.`,
	},
};

// Empty response
export const EmptyResponse: Story = {
	args: {
		children: '',
	},
};

// Custom styling
export const CustomStyling: Story = {
	args: {
		children: 'This response has custom styling applied.',
		className: 'border-2 border-primary p-4 bg-primary-content',
	},
};

// Streaming simulation (shows partial content)
export const StreamingContent: Story = {
	args: {
		children: `I'm analyzing your request and will provide a comprehensive solution.

First, let me examine the current implementation...

\`\`\`javascript
// Current implementation
function processData(items) {
  return items.map(item => {
    // Processing logic here
    return transform`,
	},
	decorators: [
		(Story) => (
			<div>
				<Story />
				<p className="text-xs text-muted-foreground mt-2">(Simulating streaming - content appears incomplete)</p>
			</div>
		),
	],
};
