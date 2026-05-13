import { create } from '@bufbuild/protobuf';
import { type Organization, OrganizationSchema } from '@polypixel/memoir-sdk/api-service/api/v1/organizations_pb';
import { describe, expect, it } from 'vitest';
import { resolveOrgPid } from './resolve';

function org(pid: string): Organization {
	return create(OrganizationSchema, { pid, name: pid, slug: pid });
}

describe('resolveOrgPid', () => {
	it('should return the cookie pid when it matches an org in the list', () => {
		const orgs = [org('alpha'), org('beta'), org('gamma')];
		expect(resolveOrgPid(orgs, 'beta')).toBe('beta');
	});

	it('should fall back to the first org when the cookie pid is not in the list', () => {
		const orgs = [org('alpha'), org('beta')];
		expect(resolveOrgPid(orgs, 'deleted-org-pid')).toBe('alpha');
	});

	it('should fall back to the first org when the cookie pid is undefined', () => {
		const orgs = [org('alpha'), org('beta')];
		expect(resolveOrgPid(orgs, undefined)).toBe('alpha');
	});

	it('should fall back to the first org when the cookie pid is empty string', () => {
		const orgs = [org('alpha'), org('beta')];
		expect(resolveOrgPid(orgs, '')).toBe('alpha');
	});

	it('should return undefined when the org list is empty and cookie is set', () => {
		expect(resolveOrgPid([], 'any-pid')).toBeUndefined();
	});

	it('should return undefined when the org list is empty and cookie is undefined', () => {
		expect(resolveOrgPid([], undefined)).toBeUndefined();
	});
});
