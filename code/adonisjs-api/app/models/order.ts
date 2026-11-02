import { belongsTo, hasMany } from '@adonisjs/lucid/orm'
import type { BelongsTo, HasMany } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { OrderSchema } from '#database/schema'
import User from '#models/user'
import Event from '#models/event'
import OrderItem from '#models/order_item'
import Ticket from '#models/ticket'

export default class Order extends WithUlid(OrderSchema, 'ord') {
  // Relationships
  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @belongsTo(() => Event)
  declare event: BelongsTo<typeof Event>

  @hasMany(() => OrderItem)
  declare items: HasMany<typeof OrderItem>

  @hasMany(() => Ticket)
  declare tickets: HasMany<typeof Ticket>

  // Scopes
  static pending() {
    return this.query().where('status', 'pending')
  }

  static forUser(userId: string) {
    return this.query().where('userId', userId)
  }
}
