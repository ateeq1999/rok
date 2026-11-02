import { defineConfig, store, drivers } from '@adonisjs/cache'

const cacheConfig = defineConfig({
  default: 'default',
  ttl: '5m',
  grace: '1h',
  graceBackoff: '30s',

  stores: {
    memoryOnly: store().useL1Layer(drivers.memory({ maxSize: '50mb' })),

    default: store()
      .useL1Layer(drivers.memory({ maxSize: '100mb', maxItems: 2000 }))
      .useL2Layer(drivers.redis({ connectionName: 'main' }))
      .useBus(drivers.redisBus({ connectionName: 'main' })),
  },
})

export default cacheConfig

declare module '@adonisjs/cache/types' {
  interface CacheStores extends InferStores<typeof cacheConfig> { }
}
