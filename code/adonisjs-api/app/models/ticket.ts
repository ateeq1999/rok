import { belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { TicketSchema } from '#database/schema'
import Event from '#models/event'
import User from '#models/user'
import TicketType from '#models/ticket_type'
import Order from '#models/order'

const SECRET_LENGTH = 64

export default class Ticket extends WithUlid(TicketSchema, 'tkt') {

  // Relationships
  @belongsTo(() => Event)
  declare event: BelongsTo<typeof Event>

  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @belongsTo(() => TicketType)
  declare ticketType: BelongsTo<typeof TicketType>

  @belongsTo(() => Order)
  declare order: BelongsTo<typeof Order>

  // Methods
  async generateSecret(): Promise<string> {
    const crypto = await import('node:crypto')
    const secret = crypto.randomBytes(SECRET_LENGTH / 2).toString('hex')
    this.secret = secret
    await this.save()
    return secret
  }

  // Scopes
  static issued() {
    return this.query().whereIn('status', ['issued', 'checked_in'])
  }

  static forUser(userId: string) {
    return this.query().where('userId', userId)
  }
}
