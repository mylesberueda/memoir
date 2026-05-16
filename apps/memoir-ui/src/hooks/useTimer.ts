import { useCallback, useEffect, useRef, useState } from 'react';

export interface TimerState {
	elapsed: number;
	isRunning: boolean;
}

export interface UseTimerReturn extends TimerState {
	start: () => void;
	stop: () => number;
	setElapsed: (seconds: number) => void;
}

export default function useTimer(): UseTimerReturn {
	const [state, setState] = useState<TimerState>({ elapsed: 0, isRunning: false });
	const startTimeRef = useRef<number | null>(null);
	const intervalRef = useRef<NodeJS.Timeout | null>(null);
	// Use a ref to track running state to avoid stale closure issues
	const isRunningRef = useRef(false);

	const start = useCallback(() => {
		// Check ref instead of state to avoid stale closures
		if (isRunningRef.current) return;

		isRunningRef.current = true;
		startTimeRef.current = Date.now();
		setState({ isRunning: true, elapsed: 0 });

		intervalRef.current = setInterval(() => {
			if (startTimeRef.current) {
				const elapsed = Math.floor((Date.now() - startTimeRef.current) / 1000);
				setState((prev) => (prev.elapsed !== elapsed ? { ...prev, elapsed } : prev));
			}
		}, 1000);
	}, []);

	const stop = useCallback(() => {
		if (!isRunningRef.current) return state.elapsed;

		if (intervalRef.current) {
			clearInterval(intervalRef.current);
			intervalRef.current = null;
		}

		const finalDuration = startTimeRef.current ? Math.floor((Date.now() - startTimeRef.current) / 1000) : state.elapsed;

		isRunningRef.current = false;
		startTimeRef.current = null;
		setState({ elapsed: finalDuration, isRunning: false });

		return finalDuration;
	}, [state.elapsed]);

	const setElapsed = useCallback((seconds: number) => {
		setState((prev) => ({ ...prev, elapsed: seconds }));
		if (isRunningRef.current && startTimeRef.current) {
			startTimeRef.current = Date.now() - seconds * 1000;
		}
	}, []);

	useEffect(() => {
		return () => {
			if (intervalRef.current) {
				clearInterval(intervalRef.current);
			}
		};
	}, []);

	return { ...state, start, stop, setElapsed };
}
