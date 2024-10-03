import { sentryVitePlugin } from '@sentry/vite-plugin';
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), sentryVitePlugin({
    org: 'smagny',
    project: 'ai-likes-human'
  })],

  define: {
    global: {},
  },

  test: {
    include: ['**/*.test.ts*'],
    environment: 'jsdom',
    setupFiles: ['./vitest-setup.js'],
    globals: true,
  },

  resolve: {
    alias: [
      { find: '@', replacement: path.resolve(__dirname, 'src') },
    ],
  },

  build: {
    sourcemap: true
  }
});