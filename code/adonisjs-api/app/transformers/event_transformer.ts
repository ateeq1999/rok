import { BaseTransformer } from '@adonisjs/core/transformers'
import type Event from '#models/event'

export type EventStatus = 'draft' | 'published' | 'cancelled'

export default class EventTransformer extends BaseTransformer<Event> {
  toObject() {
    return {
      ...this.pick(this.resource, [
        'id',
        'title',
        'description',
        'coverImageUrl',
        'location',
        'city',
        'country',
        'address',
        'latitude',
        'longitude',
        'status',
        'category',
        'capacity',
        'rsvpCount',
        'isFeatured',
        'isPublic',
      ]),
      startsAt: this.resource.startsAt.toISO()!,
      endsAt: this.resource.endsAt?.toISO() ?? null,
      salesOpensAt: this.resource.salesOpensAt?.toISO() ?? null,
      salesClosesAt: this.resource.salesClosesAt?.toISO() ?? null,
      publishedAt: this.resource.publishedAt?.toISO() ?? null,
      createdAt: this.resource.createdAt.toISO()!,
    }
  }

  /**
   * Detailed variant — includes organizer info resolved from $extras.
   * Use when you SELECT organizerInitials via a join/subquery rather than
   * fully preloading the User relation.
   */
  forDetail() {
    const extras = this.resource.$extras as { organizerInitials?: string }

    return {
      ...this.toObject(),
      organizer: this.resource.organizerId
        ? { id: this.resource.organizerId, initials: extras.organizerInitials ?? '' }
        : null,
    }
  }
}
