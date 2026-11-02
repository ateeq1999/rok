import { BaseTransformer } from '@adonisjs/core/transformers'
import type TicketType from '#models/ticket_type'

export default class TicketTypeTransformer extends BaseTransformer<TicketType> {
  toObject() {
    const sold = this.resource.quantitySold ?? 0

    return {
      ...this.pick(this.resource, [
        'id',
        'eventId',
        'name',
        'description',
        'perks',
        'price',
        'currency',
        'quantityTotal',
        'minPerOrder',
        'maxPerOrder',
        'status',
      ]),
      quantityAvailable: Math.max(0, this.resource.quantityTotal - sold),
    }
  }
}
