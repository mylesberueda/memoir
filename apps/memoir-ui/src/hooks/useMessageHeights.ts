import { useCallback, useRef, useState } from 'react';

export function useMessageHeights() {
	const heightsRef = useRef<Map<string, number>>(new Map());
	const [averageHeight, setAverageHeight] = useState(80);

	const setHeight = useCallback((id: string, height: number) => {
		heightsRef.current.set(id, height);

		// Recalculate average
		const heights = Array.from(heightsRef.current.values());
		const avg = heights.reduce((a, b) => a + b, 0) / heights.length;
		setAverageHeight(Math.round(avg));
	}, []);

	const getHeight = useCallback(
		(id: string) => {
			return heightsRef.current.get(id) || averageHeight;
		},
		[averageHeight],
	);

	const getTotalHeight = useCallback(
		(messageIds: string[]) => {
			return messageIds.reduce((total, id) => total + getHeight(id), 0);
		},
		[getHeight],
	);

	const getHeightUpTo = useCallback(
		(messageIds: string[], index: number) => {
			return messageIds.slice(0, index).reduce((total, id) => total + getHeight(id), 0);
		},
		[getHeight],
	);

	return {
		setHeight,
		getHeight,
		averageHeight,
		getTotalHeight,
		getHeightUpTo,
	};
}
