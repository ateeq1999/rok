import type { HttpContext } from '@adonisjs/core/http'
import TicketType from '#models/ticket_type'
import TicketTypeTransformer from '#transformers/ticket_type_transformer'
import { success, notFound } from '#helpers/response'

export default class TicketTypesController {
  async index({ params, response }: HttpContext) {
    const ticketTypes = await TicketType.query()
      .where('eventId', params.id)
      .where('status', 'active')
      .orderBy('price', 'asc')

    if (ticketTypes.length === 0) {
      const exists = await TicketType.query().where('eventId', params.id).first()
      if (!exists) {
        return notFound(response, 'Event not found or has no ticket types')
      }
    }

    return success(response, TicketTypeTransformer.transform(ticketTypes))
  }
}
