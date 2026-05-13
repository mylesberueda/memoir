'use client';
import { Card as DaisyCard } from 'rsc-daisyui';

export interface CardProps {
	icon?: React.ElementType;
	title: string;
	description: string;
	children: React.ReactNode;
	action?: React.ReactNode;
}

export default function Card({ icon: Icon, title, description, children, action }: CardProps) {
	return (
		<DaisyCard border className="bg-base-100">
			<DaisyCard.Body>
				<DaisyCard.Title as="h2">
					{Icon && <Icon className="h-4 w-4 text-base-content" />}
					<span>{title}</span>
				</DaisyCard.Title>
				<p className="mb-2 text-muted italic">{description}</p>
				{children}
				{action && <DaisyCard.Actions className="justify-center">{action}</DaisyCard.Actions>}
			</DaisyCard.Body>
		</DaisyCard>
	);
}
