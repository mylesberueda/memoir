'use client';

import { logout } from '@actions/auth';
import { Button, ThemePicker } from '@components';
import useAuth from '@hooks/useAuth';
import { useLayoutContext } from '@providers';
import { Bell, ChevronRight, LogOut, Menu } from 'lucide-react';
import Link from 'next/link';
import { useTransition } from 'react';
import { Avatar, Badge, Dropdown } from 'rsc-daisyui';

interface BreadcrumbItem {
	label: string;
	href?: string;
}

export default function Header() {
	const layout = useLayoutContext();
	const { user } = useAuth();
	const [isPending, startTransition] = useTransition();

	const breadcrumbs: BreadcrumbItem[] = [
		{ label: 'memoir', href: '#' },
		{ label: 'dashboard', href: '#' },
	];

	const handleLogout = () => {
		startTransition(() => {
			logout();
		});
	};

	return (
		<nav className="flex h-full items-center justify-between border-base-300 bg-base-200 px-3 sm:px-6 border-b">
			<div className="hidden max-w-[300px] items-center space-x-1 truncate font-medium text-sm sm:flex">
				{!layout.isSidebarOpen && (
					<Button
						type="button"
						className="top-4 left-4 z-[70] rounded-lg bg-base-200 p-2 lg:hidden"
						onClick={layout.toggleSidebar}>
						<Menu className="h-5 w-5 text-base-content" />
					</Button>
				)}
				{breadcrumbs.map((item, index) => (
					<div key={item.label} className="flex items-center">
						{index > 0 && <ChevronRight className="mx-1 h-4 w-4 text-base-content" />}
						{item.href ? (
							<Link href={item.href} className="text-base-content transition-colors">
								{item.label}
							</Link>
						) : (
							<span className="text-base-content">{item.label}</span>
						)}
					</div>
				))}
			</div>
			<div className="ml-auto flex items-center gap-2 sm:ml-0 sm:gap-4">
				<button
					type="button"
					className="rounded-full p-1.5 transition-colors hover:bg-base-100 sm:p-2"
					aria-label="Notifications">
					<Bell className="h-4 w-4 text-base-content sm:h-5 sm:w-5" />
				</button>
				<ThemePicker />
				{user ? (
					<Dropdown align="end">
						<Dropdown.Button className="h-9 w-9">
							<Avatar>
								<div className="w-9 rounded">
									{/** biome-ignore lint/performance/noImgElement: I don't want to cache avatars */}
									<img
										alt="avatar"
										src="https://avatars.githubusercontent.com/u/195809365?v=4"
										width={36}
										height={36}
									/>
								</div>
							</Avatar>
						</Dropdown.Button>
						<Dropdown.Menu className="mt-3 flex w-max border border-base-300 bg-base-100 shadow-md">
							<div className="flex items-center justify-center gap-4 px-8 py-4">
								<Avatar indicator="online" className="flex-[1_0_64px]">
									<div className="w-16 rounded ring ring-primary ring-offset-base-100 ring-offset-2">
										{/** biome-ignore lint/performance/noImgElement: I don't want to cache avatars */}
										<img
											alt="avatar"
											src="https://avatars.githubusercontent.com/u/195809365?v=4"
											width={64}
											height={64}
										/>
									</div>
								</Avatar>
								<div className="flex flex-[1_0_auto] flex-col gap-1">
									<span className="text-2xl">{user.name ?? user.id}</span>
									<span>{user.email ?? '—'}</span>
									<span className="text-gray-700 italic dark:text-gray-400">Founder</span>
								</div>
							</div>
							<div className="my-3 border-b border-b-base-300" />
							<DropdownItem name="Subscription" decoration="Pro" />
							<DropdownItem name="Terms & Policies" />
							<DropdownItem name="Logout" decoration={<LogOut />} onClick={handleLogout} disabled={isPending} />
						</Dropdown.Menu>
					</Dropdown>
				) : (
					<Link href="/auth/login/" className="btn btn-primary btn-sm" prefetch={false}>
						Sign In
					</Link>
				)}
			</div>
		</nav>
	);
}

interface DropdownItemProps {
	name: string;
	decoration?: React.ReactNode;
	href?: string;
	onClick?: () => void;
	disabled?: boolean;
}

function DropdownItem({ name, decoration, href, onClick, disabled }: DropdownItemProps) {
	const content = (
		<>
			<span>{name}</span>
			{typeof decoration === 'string' ? (
				<Badge>{decoration}</Badge>
			) : decoration ? (
				<Badge className="border-none" outline>
					{decoration}
				</Badge>
			) : null}
		</>
	);

	if (href) {
		return (
			<li className="flex justify-between cursor-pointer hover:bg-base-200 px-4 py-2">
				<Link href={href} className="flex justify-between w-full">
					{content}
				</Link>
			</li>
		);
	}

	if (onClick) {
		return (
			<Dropdown.Item
				className={`flex justify-between ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
				onClick={disabled ? undefined : onClick}>
				{content}
			</Dropdown.Item>
		);
	}

	return <Dropdown.Item className="flex justify-between">{content}</Dropdown.Item>;
}
