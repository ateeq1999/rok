import { BaseTransformer } from '@adonisjs/core/transformers'
import type Ticket from '#models/ticket'
import TicketTypeTransformer from '#transformers/ticket_type_transformer'

export type TicketStatus = 'issued' | 'checked_in' | 'refunded' | 'voided'

export default class TicketTransformer extends BaseTransformer<Ticket> {
  toObject() {
    const ticketType = this.resource.ticketType
    return {
      ...this.pick(this.resource, ['id', 'eventId', 'orderId', 'ticketTypeId', 'status', 'secret']),
      issuedAt: this.resource.issuedAt.toISO()!,
      ticketType: ticketType ? new TicketTypeTransformer(ticketType).toObject() : null,
    }
  }

  forList() {
    const ticketType = this.resource.ticketType
    return {
      ...this.pick(this.resource, ['id', 'eventId', 'status']),
      issuedAt: this.resource.issuedAt.toISO()!,
      ticketType: ticketType ? new TicketTypeTransformer(ticketType).toObject() : null,
    }
  }

  forScan() {
    return {
      id: this.resource.id,
      secret: this.resource.secret,
      status: this.resource.status,
    }
  }

  async forDetail() {
    return this.toObject()
  }
}
