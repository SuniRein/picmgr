/// <reference types="node" />

import path from 'node:path';
import process from 'node:process';
import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { defineConfig } from 'vite';

export default defineConfig(() => {
  const VITE_SERVER_ADDR = process.env.VITE_SERVER_ADDR;
  if (!VITE_SERVER_ADDR) {
    throw new Error('SERVER_ADDR environment variable is not set');
  }

  return {
    plugins: [
      vue(),
      tailwindcss(),
      AutoImport({
        imports: ['vue', 'vue-router', 'pinia', '@vueuse/core', { '@/router': ['P'] }],
        dirs: ['src/stores', 'src/hooks'],
        dts: 'src/auto-imports.d.ts',
        vueTemplate: true,
      }),
      Components({
        dts: 'src/components.d.ts',
      }),
    ],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src'),
      },
    },
    server: {
      proxy: {
        '/api': VITE_SERVER_ADDR,
      },
    },
  };
});
