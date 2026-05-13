import { resolve } from 'node:path';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import { coverageConfigDefaults, defineConfig } from 'vitest/config';

// Base configuration shared across all test environments
export default defineConfig({
	cacheDir: '../../node_modules/.vite/apps/memoir-ui',
	plugins: [react(), tsconfigPaths()],
	define: {
		'process.env': JSON.stringify({}),
	},
	resolve: {
		alias: {
			'@': resolve(__dirname, './src'),
			'@actions': resolve(__dirname, './src/actions'),
			'@components': resolve(__dirname, './src/components'),
			'@hooks': resolve(__dirname, './src/hooks'),
			'@providers': resolve(__dirname, './src/providers'),
			'@lib': resolve(__dirname, './src/lib'),
			'@test-utils': resolve(__dirname, './src/test-utils'),
			'server-only': resolve(__dirname, './src/test-utils/server-only-mock.ts'),
		},
	},
	test: {
		globals: true,
		watch: false,
		reporters: ['default'],
		coverage: {
			reportsDirectory: './coverage',
			provider: 'v8' as const,
			exclude: ['**/*.stories.tsx', '**/*.config.{ts,js,cjs,mjs,mts}', ...coverageConfigDefaults.exclude],
		},
		projects: [
			{
				extends: true,
				test: {
					name: 'unit',
					environment: 'jsdom',
					setupFiles: ['./vitest.setup.ts'],
					include: ['src/**/*.spec.{ts,tsx}'],
					exclude: ['src/**/*.integration.spec.{ts,tsx}'],
					testTimeout: 10000, // 10 second timeout for CI environments
					css: {
						modules: {
							classNameStrategy: 'non-scoped',
						},
					},
					server: {
						deps: {
							inline: ['katex', 'streamdown'],
						},
					},
				},
			},
			{
				extends: true,
				test: {
					name: 'integration',
					environment: 'node',
					include: ['src/**/*.integration.spec.{ts,tsx}'],
				},
			},
			{
				extends: './vitest.browser.config.ts',
				test: {
					name: 'storybook',
					environment: 'jsdom',
					setupFiles: ['./.storybook/vitest.setup.ts'],
					// Enable Storybook integration
					globals: true,
				},
			},
		],
	},
});
