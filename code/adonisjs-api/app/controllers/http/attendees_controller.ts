import type { HttpContext } from '@adonisjs/core/http'
import Event from '#models/event'
import Ticket from '#models/ticket'
import TicketTransformer from '#transformers/ticket_transformer'
import { paginated, cursor, notFound, badRequest } from '#helpers/response'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'

export default class AttendeesController {
  /**
   * Get attendees for an event with pagination (offset or cursor).
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20
   * - cursor (cursor mode): cursor string for next page
   */
  async index({ request, params, response }: HttpContext) {
    const { mode = 'offset', page = 1, limit = 20, cursor: cursorParam } = request.qs()

    const event = await Event.find(params.id)
    if (!event) {
      return notFound(response, 'Event not found')
    }

    const baseQuery = Ticket.query()
      .where('eventId', event.id)
      .preload('user', (query) => query.select('id', 'fullName', 'email'))
      .orderBy('issuedAt', 'asc')
      .orderBy('id', 'asc') // tie-breaker for cursor stability

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      const tickets = await baseQuery.paginate(page, limit)
      return paginated(response, TicketTransformer.transform(tickets), tickets.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) {
          // Apply condition for (issuedAt, id) cursor
          applyCursorCondition(cursorQuery, pos, 'issuedAt', 'asc')
        }
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (ticket) => ticket.issuedAt.toString(),
        (ticket) => ticket.id
      )

      return cursor(response, TicketTransformer.transform(items), nextCursor)
    }

    // Invalid mode
    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  /**
   * Export attendees as CSV (offset only, no pagination).
   */
  async csv({ params, response }: HttpContext) {
    const event = await Event.find(params.id)
    if (!event) {
      return notFound(response, 'Event not found')
    }

    const tickets = await Ticket.query()
      .where('eventId', event.id)
      .preload('user')
      .orderBy('issuedAt', 'asc')

    if (tickets.length === 0) {
      return badRequest(response, 'No attendees found for this event')
    }

    const header = 'Name,Email,Ticket Status,Issued At'
    const lines = tickets.map((ticket) => {
      const user = ticket.user
      const name = user.fullName ? `"${String(user.fullName).replace(/"/g, '""')}"` : ''
      const email = `"${String(user.email).replace(/"/g, '""')}"`
      const status = ticket.status ?? ''
      const issuedAt = ticket.issuedAt ? ticket.issuedAt.toISO() : ''
      return `${name},${email},${status},${issuedAt}`
    })

    const csv = [header, ...lines].join('\n')

    response.header('Content-Type', 'text/csv; charset=utf-8')
    response.header('Content-Disposition', `attachment; filename="attendees-${event.id}.csv"`)
    return response.send(csv)
  }
}
