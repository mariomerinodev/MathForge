import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
			// Permite a Vite acceder a carpetas fuera de apps/web si fuera necesario
			allow: ['../..']
		}
	}
});