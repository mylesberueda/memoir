import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { storybookTest } from '@storybook/addon-vitest/vitest-plugin';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import { coverageConfigDefaults, defineConfig } from 'vitest/config';

const dirname = typeof __dirname !== 'undefined' ? __dirname : path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	cacheDir: '../../node_modules/.vite/apps/memoir-ui',
	plugins: [
		react(),
		tsconfigPaths(),
		storybookTest({
			configDir: path.join(dirname, '.storybook'),
			// storybookScript: "pnpm nx run memoir-ui:storybook",
			// storybookUrl: "http://localhost:4400",
		}),
	],
	define: {
		'process.env': JSON.stringify({}),
	},
	test: {
		setupFiles: ['./.storybook/vitest.setup.ts'],
		exclude: ['**/node_modules/**', '**/dist/**', '**/build/**', '**/.next/**', '**/.storybook/**', '**/coverage/**'],
		watch: false,
		globals: true,
		reporters: ['default'],
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
		browser: {
			enabled: true,
			provider: 'playwright',
			headless: true,
			instances: [{ browser: 'chromium' }],
			isolate: true,
			fileParallelism: false,
		},
		coverage: {
			reportsDirectory: './coverage/storybook',
			provider: 'istanbul' as const,
			exclude: ['**/*.stories.tsx', ...coverageConfigDefaults.exclude],
		},
	},
});
