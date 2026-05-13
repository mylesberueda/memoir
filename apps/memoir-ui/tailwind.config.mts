import type { Config } from 'tailwindcss';

const CONFIG: Config = {
	content: ['./src/**/*.{ts,tsx,html}', '../../node_modules/daisyui/dist/**/*.js'],
	theme: {
		extend: {
			borderRadius: {
				lg: 'var(--radius)',
				md: 'calc(var(--radius) - 2px)',
				sm: 'calc(var(--radius) - 4px)',
			},
			colors: {},
		},
	},
	plugins: [require('tailwindcss-animate')],
};

export default CONFIG;
