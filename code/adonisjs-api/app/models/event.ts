import { hasMany } from '@adonisjs/lucid/orm'
import type { HasMany } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { EventSchema } from '#database/schema'
import TicketType from '#models/ticket_type'
import Ticket from '#models/ticket'
import Order from '#models/order'
import EventRsvp from '#models/event_rsvp'

export default class Event extends WithUlid(EventSchema, 'evt') {

  // Relationships
  @hasMany(() => TicketType)
  declare ticketTypes: HasMany<typeof TicketType>

  @hasMany(() => Ticket)
  declare tickets: HasMany<typeof Ticket>

  @hasMany(() => Order)
  declare orders: HasMany<typeof Order>

  @hasMany(() => EventRsvp)
  declare rsvps: HasMany<typeof EventRsvp>

  // Scopes
  static published() {
    return this.query().where('status', 'published').whereNull('deletedAt')
  }
}
