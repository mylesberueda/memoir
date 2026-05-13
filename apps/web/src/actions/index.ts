export { login, logout, register } from './auth';
export * from './organizations';

export type ActionResult<T> = { success: true; data: T } | { success: false; error: string };
