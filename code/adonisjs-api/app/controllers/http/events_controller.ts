import type { HttpContext } from '@adonisjs/core/http'
import cache from '@adonisjs/cache/services/main'
import Event from '#models/event'
import EventTransformer from '#transformers/event_transformer'
import { createEventSchema, updateEventSchema } from '#validators/event'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { DateTime } from 'luxon'
import {
  paginated,
  cursor,
  success,
  created,
  noContent,
  unauthorized,
  badRequest,
} from '#helpers/response'

export default class EventsController {
  /**
   * List events with pagination (offset or cursor).
   * For admin: includes events by status (no deleted).
   * For public: only published events.
   *
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20
   * - cursor (cursor mode): cursor string for next page
   * - status (admin only): filter by status
   * - city, category, dateFrom, dateTo (public only): filters
   */
  async index({ request, response }: HttpContext) {
    const {
      mode = 'offset',
      page = 1,
      limit = 20,
      cursor: cursorParam,
      status,
      city,
      category,
      dateFrom,
      dateTo,
    } = request.qs()

    const isAdminRoute = request.url().includes('/admin')

    // Base query: exclude soft-deleted events
    const baseQuery = Event.query().whereNull('deletedAt')

    // Admin vs public filters
    if (isAdminRoute) {
      baseQuery.orderBy('createdAt', 'desc')
      if (status) baseQuery.where('status', status)
    } else {
      baseQuery.where('status', 'published').orderBy('startsAt', 'asc').orderBy('id', 'asc')
      if (city) baseQuery.where('city', city)
      if (category) baseQuery.where('category', category)
      if (dateFrom) baseQuery.where('startsAt', '>=', new Date(dateFrom).toISOString())
      if (dateTo) baseQuery.where('startsAt', '<=', new Date(dateTo).toISOString())
    }

    const cacheKey = `events:${isAdminRoute ? 'admin' : 'pub'}:${mode}:p${page}:l${limit}:${cursorParam ?? ''}:${city ?? ''}:${category ?? ''}:${dateFrom ?? ''}:${dateTo ?? ''}`

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      if (!isAdminRoute) {
        const cached = await cache.getOrSet({
          key: cacheKey,
          tags: ['events'],
          factory: async () => {
            const events = await baseQuery.paginate(page, limit)
            return { data: EventTransformer.transform(events.all()), meta: events.getMeta() }
          },
        })
        return paginated(response, cached.data, cached.meta)
      }
      const events = await baseQuery.paginate(page, limit)
      return paginated(response, EventTransformer.transform(events.all()), events.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()

      // Ensure deterministic ordering for cursor
      if (!isAdminRoute) {
        cursorQuery.orderBy('startsAt', 'asc').orderBy('id', 'asc')
      } else {
        cursorQuery.orderBy('createdAt', 'desc').orderBy('id', 'desc')
      }

      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) {
          const orderColumn = isAdminRoute ? 'createdAt' : 'startsAt'
          const orderDirection = isAdminRoute ? 'desc' : 'asc'
          applyCursorCondition(cursorQuery, pos, orderColumn, orderDirection)
        }
      }

      if (!isAdminRoute) {
        const cached = await cache.getOrSet({
          key: cacheKey,
          tags: ['events'],
          factory: async () => {
            const rows = await cursorQuery.limit(limit + 1)
            const { items, nextCursor: nc } = paginateCursor(
              rows, limit,
              (event) => event.startsAt.toString(),
              (event) => event.id
            )
            return { data: items.map((item) => new EventTransformer(item).toObject()), nextCursor: nc }
          },
        })
        return cursor(response, cached.data, cached.nextCursor)
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (event) => (isAdminRoute ? event.createdAt.toString() : event.startsAt.toString()),
        (event) => event.id
      )

      const transformed = items.map((item) => new EventTransformer(item).toObject())
      return cursor(response, transformed, nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  /**
   * Get a single event (public, must be published).
   */
  async show({ params, response }: HttpContext) {
    const event = await Event.query()
      .where('id', params.id)
      .whereNull('deletedAt')
      .firstOrFail()

    return success(response, EventTransformer.transform(event).useVariant('forDetail'))
  }

  /**
   * Create a new event (authenticated users only).
   */
  async store({ request, auth, response }: HttpContext) {
    const user = auth.user
    if (!user) {
      return unauthorized(response, 'Unauthorized')
    }

    const data = await request.validateUsing(createEventSchema)

    const event = await Event.create({
      ...data,
      organizerId: user.id,
      latitude: data.latitude != null ? String(data.latitude) : null,
      longitude: data.longitude != null ? String(data.longitude) : null,
    })

    if (data.status === 'published' && !event.publishedAt) {
      event.publishedAt = DateTime.now()
      await event.save()
    }

    await cache.deleteByTag({ tags: ['events'] })
    return created(response, EventTransformer.transform(event))
  }

  /**
   * Update an existing event.
   */
  async update({ params, request, response }: HttpContext) {
    const data = await request.validateUsing(updateEventSchema)

    const event = await Event.query()
      .where('id', params.id)
      .whereNull('deletedAt')
      .firstOrFail()

    event.merge({
      ...data,
      latitude: data.latitude != null ? String(data.latitude) : undefined,
      longitude: data.longitude != null ? String(data.longitude) : undefined,
    })

    if (data.status === 'published' && !event.publishedAt) {
      event.publishedAt = DateTime.now()
    }

    await event.save()
    await cache.deleteByTag({ tags: ['events'] })

    return success(response, EventTransformer.transform(event))
  }

  /**
   * Soft-delete an event.
   */
  async destroy({ params, response }: HttpContext) {
    const event = await Event.query()
      .where('id', params.id)
      .whereNull('deletedAt')
      .firstOrFail()

    event.deletedAt = DateTime.now()
    await event.save()
    await cache.deleteByTag({ tags: ['events'] })

    return noContent(response)
  }
}
