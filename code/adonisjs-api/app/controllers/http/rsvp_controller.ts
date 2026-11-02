import type { HttpContext } from '@adonisjs/core/http'
import Event from '#models/event'
import EventRsvp from '#models/event_rsvp'
import RsvpTransformer from '#transformers/rsvp_transformer'
import pointsService from '#services/points_service'
import { RSVP_POINTS } from '#helpers/formats'
import { success, unauthorized, conflict, notFound } from '#helpers/response'

export default class RsvpController {
  async rsvp({ auth, params, response }: HttpContext) {
    const user = auth.user
    if (!user) {
      return unauthorized(response, 'Login required')
    }

    const event = await Event.query()
      .where('id', params.id)
      .where('status', 'published')
      .whereNull('deletedAt')
      .firstOrFail()

    // Check for duplicate RSVP
    const existingRsvp = await EventRsvp.query()
      .where('eventId', event.id)
      .where('userId', user.id)
      .first()

    if (existingRsvp) {
      return conflict(response, 'You have already RSVP\'d to this event')
    }

    // Create RSVP record
    await EventRsvp.create({
      eventId: event.id,
      userId: user.id,
    })

    event.rsvpCount = (event.rsvpCount ?? 0) + 1
    await event.save()

    await pointsService.earn(user.id, RSVP_POINTS, 'rsvp', event.id, 'event').catch(() => { })

    return success(response, RsvpTransformer.transform({ rsvped: true, rsvpCount: event.rsvpCount }))
  }

  async cancel({ auth, params, response }: HttpContext) {
    const user = auth.user
    if (!user) {
      return unauthorized(response, 'Login required')
    }

    const event = await Event.query()
      .where('id', params.id)
      .whereNull('deletedAt')
      .firstOrFail()

    const rsvp = await EventRsvp.query()
      .where('eventId', event.id)
      .where('userId', user.id)
      .first()

    if (!rsvp) {
      return notFound(response, 'You have not RSVP\'d to this event')
    }

    await rsvp.delete()

    const current = event.rsvpCount ?? 0
    if (current > 0) {
      event.rsvpCount = current - 1
      await event.save()
    }

    return success(response, RsvpTransformer.transform({ rsvped: false, rsvpCount: event.rsvpCount ?? 0 }))
  }
}
