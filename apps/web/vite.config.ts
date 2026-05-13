import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';
import { defineConfig } from 'vitest/config';

export default defineConfig(async (_env) => {
	return {
		cacheDir: '../../node_modules/.vite/apps/web',
		plugins: [react(), tsconfigPaths()],
	};
});
