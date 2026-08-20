import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  // Tauri phục vụ file từ đĩa nên mọi đường dẫn phải tương đối.
  base: './',
  server: { port: 5273, strictPort: true },
  build: { target: 'es2022', outDir: 'dist' },
});
