import { DateTime } from 'luxon'
import { column, belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { PointLedgerSchema } from '#database/schema'
import User from '#models/user'

export default class PointLedger extends WithUlid(PointLedgerSchema, 'ptl') {
  @column()
  declare userId: string

  @column()
  declare amount: number

  @column()
  declare balanceAfter: number

  @column()
  declare reason: string

  @column()
  declare refId: string | null

  @column()
  declare refType: string | null

  @column.dateTime({ autoCreate: true })
  declare createdAt: DateTime

  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>
}
