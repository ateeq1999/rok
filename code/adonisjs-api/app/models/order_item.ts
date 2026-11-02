import { belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { OrderItemSchema } from '#database/schema'
import Order from '#models/order'
import TicketType from '#models/ticket_type'

export default class OrderItem extends WithUlid(OrderItemSchema, 'oi') {
  // Relationships
  @belongsTo(() => Order)
  declare order: BelongsTo<typeof Order>

  @belongsTo(() => TicketType)
  declare ticketType: BelongsTo<typeof TicketType>
}
