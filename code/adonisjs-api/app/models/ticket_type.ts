import { belongsTo, hasMany } from '@adonisjs/lucid/orm'
import type { BelongsTo, HasMany } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { TicketTypeSchema } from '#database/schema'
import Event from '#models/event'
import Ticket from '#models/ticket'
import OrderItem from '#models/order_item'

export default class TicketType extends WithUlid(TicketTypeSchema, 'tt') {

  // Relationships
  @belongsTo(() => Event)
  declare event: BelongsTo<typeof Event>

  @hasMany(() => Ticket)
  declare tickets: HasMany<typeof Ticket>

  @hasMany(() => OrderItem)
  declare orderItems: HasMany<typeof OrderItem>

  // Computed
  get quantityAvailable(): number {
    return this.quantityTotal - (this.quantitySold ?? 0)
  }

  // Scopes
  static active() {
    return this.query().where('status', 'active')
  }
}
