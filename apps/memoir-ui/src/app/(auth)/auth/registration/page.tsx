'use client';

import { redirect } from 'next/navigation';

// This page now redirects to the login page in registration mode
// Kept for backwards compatibility with existing links
export default function RegistrationPage() {
	redirect('/auth/login?mode=register');
}
