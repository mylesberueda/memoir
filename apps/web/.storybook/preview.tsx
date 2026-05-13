import type { Preview } from '@storybook/nextjs';
import '../src/app/global.css';

const preview: Preview = {
	parameters: {
		nextjs: {
			appDirectory: true,
		},
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i,
			},
		},
		backgrounds: {
			default: 'light',
			values: [
				{
					name: 'light',
					value: '#ffffff',
				},
				{
					name: 'dark',
					value: '#1f2937',
				},
			],
		},
		viewport: {
			viewports: {
				mobile: {
					name: 'Mobile',
					styles: {
						width: '375px',
						height: '667px',
					},
				},
				tablet: {
					name: 'Tablet',
					styles: {
						width: '768px',
						height: '1024px',
					},
				},
				desktop: {
					name: 'Desktop',
					styles: {
						width: '1024px',
						height: '768px',
					},
				},
			},
		},
	},
	decorators: [
		(Story, context) => {
			// Get the current background from Storybook's background addon
			const background = context.globals.backgrounds?.value;
			const isDark = background === '#1f2937';
			const theme = isDark ? 'dark' : 'light';

			return (
				<div data-theme={theme} className="bg-base-100 p-4">
					<Story />
				</div>
			);
		},
	],
};

export default preview;
