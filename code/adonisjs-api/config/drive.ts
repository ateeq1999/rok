import app from '@adonisjs/core/services/app'
import { defineConfig, services } from '@adonisjs/drive'

export default defineConfig({
  default: 'fs' as const,

  services: {
    fs: services.fs({
      location: app.makePath('storage'),
      serveFiles: true,
      routeBasePath: '/uploads',
      visibility: 'public',
    }),
  },
})
