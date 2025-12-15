import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// Plugin to inject document polyfill into workers
function workerPolyfillPlugin() {
  return {
    name: 'worker-polyfill',
    transform(code: string, id: string) {
      // Inject polyfill into all worker-related code
      if (id.includes('worker') || code.includes('wasm_thread_entry_point')) {
        const polyfill = `
if (typeof document === 'undefined') {
  globalThis.document = {
    getElementById: () => null,
    createElement: () => ({}),
    body: {},
    head: {},
    addEventListener: () => {},
    removeEventListener: () => {},
    querySelector: () => null,
    querySelectorAll: () => [],
  };
}
if (typeof window === 'undefined') {
  globalThis.window = globalThis;
}
`
        return polyfill + code
      }
      return code
    }
  }
}

export default defineConfig({
  plugins: [react(), workerPolyfillPlugin()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  // Critical for WASM - polyfill global for worker environments
  define: {
    global: 'globalThis',
    'process.env': '{}',
  },
  // Prevent Vite from optimizing @linera/client (breaks WASM worker initialization)
  optimizeDeps: {
    exclude: ['@linera/client'],
    esbuildOptions: {
      loader: {
        '.js': 'jsx',
      },
    },
  },
  // Worker configuration to prevent DOM access errors
  worker: {
    format: 'es',
    plugins: () => [],
  },
  server: {
    port: 5173,
    // COOP/COEP headers prevent SharedArrayBuffer errors
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'credentialless',
    },
    proxy: {
      '/graphql': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
  // Also add headers for preview/production builds
  preview: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'credentialless',
    },
  },
})
