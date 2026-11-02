import type { DateTime } from 'luxon'
import type { BaseModel } from '@adonisjs/lucid/orm'
import type { NormalizeConstructor } from '@adonisjs/core/types/helpers'

import { column } from '@adonisjs/lucid/orm'

/**
 * A mixin for models that adds `createdAt` and `updatedAt` columns
 */
export const WithTimestamps = <Model extends NormalizeConstructor<typeof BaseModel>>(
  superclass: Model,
) => {
  class WithTimestampsClass extends superclass {
    @column.dateTime({ autoCreate: true }) declare createdAt: DateTime
    @column.dateTime({ autoCreate: true, autoUpdate: true }) declare updatedAt: DateTime
  }

  return WithTimestampsClass
}
