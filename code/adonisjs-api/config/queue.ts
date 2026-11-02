import env from '#start/env'
import { defineConfig, drivers, exponentialBackoff } from '@adonisjs/queue'

export default defineConfig({
  default: env.get('QUEUE_DRIVER', 'redis'),

  adapters: {
    redis: drivers.redis({ connectionName: 'main' }),
    database: drivers.database({ connectionName: 'pg' }),
    sync: drivers.sync(),
  },

  worker: {
    concurrency: 5,
    idleDelay: '2s',
    stalledThreshold: '30s',
    stalledInterval: '30s',
    maxStalledCount: 1,
    gracefulShutdown: true,
  },

  retry: {
    maxRetries: 3,
    backoff: exponentialBackoff({ baseDelay: '5s', maxDelay: '2m' }),
  },

  queues: {
    emails: {
      retry: { maxRetries: 5, backoff: exponentialBackoff({ baseDelay: '10s', maxDelay: '5m' }) },
    },
  },

  locations: ['./app/jobs/**/*.{ts,js}'],
})