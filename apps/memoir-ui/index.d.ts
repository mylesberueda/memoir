/* eslint-disable @typescript-eslint/no-explicit-any */
declare module '*.svg' {
	// biome-ignore lint/suspicious/noExplicitAny: SVG module declarations require any type
	const content: any;
	// biome-ignore lint/suspicious/noExplicitAny: SVG module declarations require any type
	export const ReactComponent: any;
	export default content;
}
