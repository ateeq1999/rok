// app/controllers/tickets_controller.ts
import type { HttpContext } from '@adonisjs/core/http'
import { DateTime } from 'luxon'
import Ticket from '#models/ticket'
import TicketTransformer from '#transformers/ticket_transformer'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { scanTicketSchema } from '#validators/ticket'
import { cursor, paginated, success, notFound, unauthorized, conflict, unprocessable, badRequest } from '#helpers/response'

export default class TicketsController {
  /**
   * Get user's tickets with pagination (offset or cursor).
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20 (max 50)
   * - cursor (cursor mode): cursor string for next page
   */
  async mine({ auth, request, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const {
      mode = 'offset',
      page = 1,
      limit = 20,
      cursor: rawCursor,
    } = request.qs()
    // enforce max 50
    const baseQuery = Ticket.query()
      .where('userId', user.id)
      .whereIn('status', ['issued', 'checked_in'])
      .preload('ticketType')
      .orderBy('issuedAt', 'desc')
      .orderBy('id', 'desc')

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      const tickets = await baseQuery.paginate(page, limit)
      return paginated(response, TicketTransformer.transform(tickets).useVariant('toObject'), tickets.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      if (rawCursor) {
        const pos = decodeCursor(rawCursor)
        if (pos) {
          applyCursorCondition(cursorQuery, pos, 'issuedAt', 'desc')
        }
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (ticket) => ticket.issuedAt.toISO(),
        (ticket) => ticket.id
      )

      const transformed = TicketTransformer.transform(items).useVariant('toObject')
      return cursor(response, transformed, nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  async show({ params, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()

    const ticket = await Ticket.query()
      .where('id', params.id)
      .where('userId', user.id)
      .preload('ticketType')
      .first()

    if (!ticket) {
      return notFound(response, 'Ticket not found')
    }

    return success(response, TicketTransformer.transform(ticket).useVariant('forDetail'))
  }

  async scan({ auth, request, response }: HttpContext) {
    const admin = auth.getUserOrFail()
    if (admin.role !== 'admin') {
      return unauthorized(response, 'Unauthorized')
    }

    const { secret } = await request.validateUsing(scanTicketSchema)

    const ticket = await Ticket.query()
      .where('secret', secret)
      .preload('ticketType')
      .first()

    if (!ticket) {
      return notFound(response, 'Ticket not found')
    }

    if (ticket.status === 'checked_in') {
      return conflict(response, 'Ticket already checked in')
    }

    if (ticket.status === 'refunded' || ticket.status === 'voided') {
      return unprocessable(response, `Ticket is ${ticket.status}`)
    }

    ticket.status = 'checked_in'
    ticket.checkedInAt = DateTime.now()
    ticket.checkedInBy = admin.id
    await ticket.save()

    return success(response, TicketTransformer.transform(ticket).useVariant('forScan'))
  }
}
