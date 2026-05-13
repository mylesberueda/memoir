'use client';

import { useCallback, useEffect, useState } from 'react';

const THEME_STORAGE_KEY = 'x-startup-theme';

export default function ThemePicker() {
	const [theme, setTheme] = useState<'light' | 'dark'>('dark');
	const [isLoaded, setIsLoaded] = useState(false);

	// Load theme from localStorage on component mount
	useEffect(() => {
		const savedTheme = localStorage.getItem(THEME_STORAGE_KEY) as 'light' | 'dark';
		const initialTheme = savedTheme || 'dark';

		const html = document.querySelector('html');
		if (html) {
			html.setAttribute('data-theme', initialTheme);
		}

		setTheme(initialTheme);
		setIsLoaded(true);
	}, []);

	const handleOnChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		const html = document.querySelector('html');
		if (!html) {
			throw Error('unable to find html tag');
		}

		const newTheme = e.target.checked ? 'light' : 'dark';

		html.setAttribute('data-theme', newTheme);
		setTheme(newTheme);
		localStorage.setItem(THEME_STORAGE_KEY, newTheme);
	}, []);

	// Don't render until theme is loaded to prevent flash
	if (!isLoaded) {
		return null;
	}

	return (
		<div>
			<input
				type="checkbox"
				checked={theme === 'light'}
				className="toggle theme-controller"
				onChange={handleOnChange}
			/>
		</div>
	);
}
