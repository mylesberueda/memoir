import type { StorybookConfig } from '@storybook/nextjs-vite';

const config: StorybookConfig = {
	stories: ['../**/*.@(mdx|stories.@(js|jsx|ts|tsx))'],
	addons: ['@storybook/addon-docs', '@storybook/addon-vitest'],
	framework: {
		name: '@storybook/nextjs-vite',
		options: {},
	},
	typescript: {
		check: false,
		reactDocgen: 'react-docgen-typescript',
		reactDocgenTypescriptOptions: {
			shouldExtractLiteralValuesFromEnum: true,
			propFilter: (prop) => (prop.parent ? !/node_modules/.test(prop.parent.fileName) : true),
		},
	},
};

export default config;

// To customize your webpack configuration you can use the webpackFinal field.
// Check https://storybook.js.org/docs/react/builders/webpack#extending-storybooks-webpack-config
// and https://nx.dev/recipes/storybook/custom-builder-configs
