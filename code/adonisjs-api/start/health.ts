import { HealthChecks, DiskSpaceCheck, MemoryHeapCheck, MemoryRSSCheck } from '@adonisjs/core/health'
import db from '@adonisjs/lucid/services/db'
import { DbCheck, DbConnectionCountCheck } from '@adonisjs/lucid/database'
import redis from '@adonisjs/redis/services/main'
import { RedisCheck, RedisMemoryUsageCheck } from '@adonisjs/redis'

export const healthChecks = new HealthChecks().register([
  new DiskSpaceCheck().cacheFor('1 hour'),
  new MemoryHeapCheck(),
  new MemoryRSSCheck(),
  new DbCheck(db.connection()),
  new DbConnectionCountCheck(db.connection()),
  new RedisCheck(redis.connection()),
  new RedisMemoryUsageCheck(redis.connection()),
])
