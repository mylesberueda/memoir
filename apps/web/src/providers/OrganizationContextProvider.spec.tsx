import '@testing-library/jest-dom/vitest';

import type { Organization, ResourcePermission } from '@polypixel/proto-ts/api-service/api/v1/organizations_pb';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useContext } from 'react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { OrganizationContext, OrganizationContextProvider, useOrganizations } from './OrganizationContextProvider';

const mockPush = vi.fn();

vi.mock('next/navigation', () => ({
	useRouter: () => ({
		push: mockPush,
		replace: vi.fn(),
		refresh: vi.fn(),
		back: vi.fn(),
		forward: vi.fn(),
		prefetch: vi.fn(),
	}),
}));

vi.mock('@actions/organizations', () => ({
	getOrganizationByPid: vi.fn(),
}));

vi.mock('@lib/grpc/transport', () => ({
	setOrganizationContext: vi.fn().mockResolvedValue(undefined),
}));

const { getOrganizationByPid } = await import('@actions/organizations');
const { setOrganizationContext } = await import('@lib/grpc/transport');
const mockGetOrgByPid = vi.mocked(getOrganizationByPid);
const mockSetOrgContext = vi.mocked(setOrganizationContext);

function makeOrg(pid: string, name: string): Organization {
	return { pid, name, slug: pid } as unknown as Organization;
}

function makePerm(read: boolean, write: boolean, execute: boolean): ResourcePermission {
	return { read, write, execute } as unknown as ResourcePermission;
}

function TestConsumer() {
	const ctx = useContext(OrganizationContext);

	if (!ctx) {
		return <div data-testid="no-context">No context</div>;
	}

	return (
		<div data-testid="consumer">
			<div data-testid="current-org-pid">{ctx.currentOrgPid ?? 'none'}</div>
			<div data-testid="can-agents-read">{ctx.can('agents', 'read').toString()}</div>
			<div data-testid="can-agents-write">{ctx.can('agents', 'write').toString()}</div>
			<div data-testid="can-billing-read">{ctx.can('billing', 'read').toString()}</div>
			<div data-testid="can-billing-write">{ctx.can('billing', 'write').toString()}</div>
			<div data-testid="can-members-write">{ctx.can('members', 'write').toString()}</div>
			<div data-testid="can-unknown-read">{ctx.can('unknown-resource', 'read').toString()}</div>
			<button data-testid="switch-org" type="button" onClick={() => ctx.setCurrentOrg('org-2')}>
				Switch Org
			</button>
			<button data-testid="reset-first" type="button" onClick={() => ctx.resetToFirstOrg()}>
				Reset First
			</button>
		</div>
	);
}

const orgs = [makeOrg('org-1', 'First Org'), makeOrg('org-2', 'Team Org')];

const ownerPermissions = {
	agents: makePerm(true, true, true),
	billing: makePerm(true, true, true),
	members: makePerm(true, true, true),
};

describe('OrganizationContextProvider', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mockPush.mockClear();
	});

	describe('initial render with server-resolved props', () => {
		it('should grant permissions on first render when initialPermissions is provided', () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('can-agents-read')).toHaveTextContent('true');
			expect(screen.getByTestId('can-billing-write')).toHaveTextContent('true');
			expect(screen.getByTestId('can-members-write')).toHaveTextContent('true');
		});

		it('should respect specific permission values from initialPermissions', () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={{
						agents: makePerm(true, false, true),
						billing: makePerm(false, false, false),
					}}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('can-agents-read')).toHaveTextContent('true');
			expect(screen.getByTestId('can-agents-write')).toHaveTextContent('false');
			expect(screen.getByTestId('can-billing-read')).toHaveTextContent('false');
		});

		it('should deny unknown resource types', () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('can-unknown-read')).toHaveTextContent('false');
		});

		it('should deny all permissions when initialPermissions is undefined', () => {
			render(
				<OrganizationContextProvider organizations={orgs} initialOrgPid="org-1" cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('can-agents-read')).toHaveTextContent('false');
			expect(screen.getByTestId('can-billing-read')).toHaveTextContent('false');
		});

		it('should use initialOrgPid as currentOrgPid', () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-2"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-2">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-2');
		});
	});

	describe('cookie reconciliation', () => {
		it('should not write the cookie when server-resolved pid matches cookie', async () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			await waitFor(() => {
				expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-1');
			});

			expect(mockSetOrgContext).not.toHaveBeenCalled();
		});

		it('should write the cookie when server-resolved pid differs from cookie', async () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="stale-pid">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			await waitFor(() => {
				expect(mockSetOrgContext).toHaveBeenCalledWith('org-1');
			});
		});

		it('should write the cookie when there was no cookie pid', async () => {
			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}>
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			await waitFor(() => {
				expect(mockSetOrgContext).toHaveBeenCalledWith('org-1');
			});
		});

		it('should not write the cookie when there is no resolved org', async () => {
			render(
				<OrganizationContextProvider organizations={[]}>
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('current-org-pid')).toHaveTextContent('none');
			expect(mockSetOrgContext).not.toHaveBeenCalled();
		});
	});

	describe('org switching', () => {
		it('should fetch permissions, set cookie, and update state when switching orgs', async () => {
			const teamPermissions = {
				agents: makePerm(true, true, true),
				billing: makePerm(false, false, false),
			};

			mockGetOrgByPid.mockResolvedValue({
				success: true,
				data: { organization: makeOrg('org-2', 'Team Org'), userRole: 'member', permissions: teamPermissions },
			} as never);

			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('can-billing-read')).toHaveTextContent('true');

			const user = userEvent.setup();
			await user.click(screen.getByTestId('switch-org'));

			await waitFor(() => {
				expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-2');
			});

			expect(mockSetOrgContext).toHaveBeenCalledWith('org-2');
			expect(mockGetOrgByPid).toHaveBeenCalledWith('org-2');
			expect(screen.getByTestId('can-billing-read')).toHaveTextContent('false');
			expect(mockPush).toHaveBeenCalledWith('/dashboard');
		});

		it('should not switch when getOrganizationByPid fails', async () => {
			mockGetOrgByPid.mockResolvedValue({ success: false, error: 'forbidden' } as never);

			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-1"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-1">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			const user = userEvent.setup();
			await user.click(screen.getByTestId('switch-org'));

			await waitFor(() => {
				expect(mockGetOrgByPid).toHaveBeenCalledWith('org-2');
			});

			expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-1');
			expect(mockSetOrgContext).not.toHaveBeenCalled();
			expect(mockPush).not.toHaveBeenCalled();
		});

		it('should reset to first org when resetToFirstOrg is called', async () => {
			mockGetOrgByPid.mockResolvedValue({
				success: true,
				data: { organization: makeOrg('org-1', 'First Org'), userRole: 'owner', permissions: ownerPermissions },
			} as never);

			render(
				<OrganizationContextProvider
					organizations={orgs}
					initialOrgPid="org-2"
					initialPermissions={ownerPermissions}
					cookieOrgPid="org-2">
					<TestConsumer />
				</OrganizationContextProvider>,
			);

			expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-2');

			const user = userEvent.setup();
			await user.click(screen.getByTestId('reset-first'));

			await waitFor(() => {
				expect(screen.getByTestId('current-org-pid')).toHaveTextContent('org-1');
			});

			expect(mockPush).toHaveBeenCalledWith('/dashboard');
		});
	});

	describe('useOrganizations hook', () => {
		it('should throw when used outside provider', () => {
			const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

			function BadConsumer() {
				useOrganizations();
				return null;
			}

			expect(() => render(<BadConsumer />)).toThrow(
				'useOrganizations must be used within an OrganizationContextProvider',
			);

			consoleSpy.mockRestore();
		});
	});
});
