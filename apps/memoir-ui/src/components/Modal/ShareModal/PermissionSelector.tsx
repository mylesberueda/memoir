'use client';

// Permission bitfield constants: 1=read, 2=write, 4=execute
export const PERM_READ = 1;
export const PERM_WRITE = 2;
export const PERM_EXECUTE = 4;

interface PermissionSelectorProps {
	value: number;
	onChange: (permissions: number) => void;
	disabled?: boolean;
}

function hasFlag(bitfield: number, flag: number): boolean {
	return (bitfield & flag) !== 0;
}

function toggleFlag(bitfield: number, flag: number, on: boolean): number {
	return on ? bitfield | flag : bitfield & ~flag;
}

const PERMISSION_FLAGS = [
	{ flag: PERM_READ, label: 'Read', shortLabel: 'R', description: 'View the resource' },
	{ flag: PERM_WRITE, label: 'Write', shortLabel: 'W', description: 'Edit the resource' },
	{ flag: PERM_EXECUTE, label: 'Execute', shortLabel: 'X', description: 'Use the resource' },
] as const;

export default function PermissionSelector({ value, onChange, disabled }: PermissionSelectorProps) {
	return (
		<div id="permission_selector__container" className="flex gap-2">
			{PERMISSION_FLAGS.map(({ flag, label, shortLabel }) => {
				const active = hasFlag(value, flag);
				return (
					<button
						key={flag}
						type="button"
						disabled={disabled}
						onClick={() => onChange(toggleFlag(value, flag, !active))}
						className={`btn btn-xs ${active ? 'btn-primary' : 'btn-ghost'}`}
						title={label}>
						{shortLabel}
					</button>
				);
			})}
		</div>
	);
}

export function PermissionBadges({ permissions }: { permissions: number }) {
	return (
		<div className="flex gap-1">
			{PERMISSION_FLAGS.map(({ flag, shortLabel, label }) => (
				<span
					key={flag}
					className={`badge badge-xs ${hasFlag(permissions, flag) ? 'badge-primary' : 'badge-ghost opacity-30'}`}
					title={label}>
					{shortLabel}
				</span>
			))}
		</div>
	);
}
