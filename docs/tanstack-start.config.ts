import { defineConfig } from '@tanstack/start/config'
import mdx from '@mdx-js/rollup'

export default defineConfig({
  vite: {
    plugins: [
      mdx({
        providerImportSource: '@mdx-js/react'
      })
    ]
  },
  appDirectory: 'src',
  routesDirectory: 'src/routes',
  outDir: 'dist',
  publicDir: 'public'
})
