import { svelteTesting } from '@testing-library/svelte/vite';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],

	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
	  port: 1420,
	  strictPort: true,
	  host: host || false,
	  hmr: host
		? {
			protocol: "ws",
			host,
			port: 1421,
		  }
		: undefined,
	  watch: {
		// 3. tell vite to ignore watching `src-tauri`
		ignored: ["**/src-tauri/**"],
	  },
	},
	test: {
		workspace: [
			{
				extends: './vite.config.ts',
				plugins: [svelteTesting()],

				test: {
					name: 'client',
					environment: 'jsdom',
					clearMocks: true,
					include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['src/lib/server/**'],
					setupFiles: ['./vitest-setup-client.ts']
				}
			},
			{
				extends: './vite.config.ts',

				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});
