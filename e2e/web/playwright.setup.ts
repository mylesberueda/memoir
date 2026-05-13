// Suppress console output during tests unless DEBUG is set
if (!process.env.DEBUG && !process.env.VERBOSE) {
	global.console = {
		...console,
		log: () => {},
		debug: () => {},
		info: () => {},
		warn: () => {},
		// Keep error to see actual issues
		error: console.error,
	};
}

// Suppress Playwright's host validation messages
process.env.PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = '1';
