import { fileURLToPath, URL } from 'node:url'

import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

const projectRoot = fileURLToPath(new URL('..', import.meta.url))

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const rootEnv = loadEnv(mode, projectRoot, '')
  const backendHost = rootEnv.BACKEND_HOST || '127.0.0.1'
  const backendPort = rootEnv.BACKEND_PORT || '3000'
  const proxy = {
    '/api': {
      target: `http://${backendHost}:${backendPort}`,
      changeOrigin: true,
      rewrite: (path: string) => path.replace(/^\/api/, ''),
    },
  }

  return {
    envDir: projectRoot,
    plugins: [
      vue(),
      vueDevTools(),
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
    server: {
      proxy,
    },
    preview: {
      proxy,
    },
  }
})
