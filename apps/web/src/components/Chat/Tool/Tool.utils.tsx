import { Badge } from '@components';
import type { ToolUIPart } from 'ai';
import { CheckCircleIcon, CircleIcon, ClockIcon, XCircleIcon } from 'lucide-react';

export const getStatusBadge = (status: ToolUIPart['state']) => {
	const labels = {
		'input-streaming': 'Pending',
		'input-available': 'Running',
		'output-available': 'Completed',
		'output-error': 'Error',
	} as const;

	const icons = {
		'input-streaming': <CircleIcon className="size-4" />,
		'input-available': <ClockIcon className="size-4 animate-pulse" />,
		'output-available': <CheckCircleIcon className="size-4 text-green-600" />,
		'output-error': <XCircleIcon className="size-4 text-red-600" />,
	} as const;

	return (
		<Badge className="rounded-full text-xs">
			{icons[status]}
			{labels[status]}
		</Badge>
	);
};

export const getBorderColor = (status: ToolUIPart['state']) => {
	const borderColors = {
		'input-streaming': 'border-info',
		'input-available': 'border-info',
		'output-available': 'border-success',
		'output-error': 'border-error',
	} as const;

	return borderColors[status];
};
