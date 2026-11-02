import vine from '@vinejs/vine'
import type { EventStatus } from '#transformers/event_transformer'

export type { EventStatus }

const EVENT_STATUSES: EventStatus[] = ['draft', 'published', 'cancelled']

// VineJS date() only accepts YYYY-MM-DD by default; explicitly add datetime formats
const DT_FORMATS = [
  'YYYY-MM-DDTHH:mm:ss.SSS[Z]',
  'YYYY-MM-DDTHH:mm:ss[Z]',
  'YYYY-MM-DDTHH:mm:ss',
  'YYYY-MM-DDTHH:mm',
  'YYYY-MM-DD',
]

export const listEventsSchema = vine.create({
  city: vine.string().trim().optional(),
  category: vine.string().trim().optional(),
  dateFrom: vine.date().optional(),
  dateTo: vine.date().optional(),
  limit: vine.number().withoutDecimals().min(1).max(100).optional(),
  cursor: vine.string().optional(),
})

export const adminListEventsSchema = vine.compile(
  vine.object({
    page: vine.number().withoutDecimals().min(1).optional(),
    perPage: vine.number().withoutDecimals().min(1).max(100).optional(),
    status: vine.string().trim().optional(),
  })
)

export const adminListAttendeesSchema = vine.compile(
  vine.object({
    page: vine.number().withoutDecimals().min(1).optional(),
    perPage: vine.number().withoutDecimals().min(1).max(100).optional(),
  })
)

export const createEventSchema = vine.create({
  title: vine.string().trim().minLength(3).maxLength(200),
  description: vine.string().trim().maxLength(5000).optional(),
  coverImageUrl: vine.string().url().optional(),
  location: vine.string().trim().minLength(3).maxLength(300),
  city: vine.string().trim().minLength(2).maxLength(100),
  country: vine.string().trim().minLength(2).maxLength(100).optional(),
  address: vine.string().trim().maxLength(500).optional(),
  latitude: vine.number().min(-90).max(90).optional(),
  longitude: vine.number().min(-180).max(180).optional(),
  status: vine.enum(EVENT_STATUSES).optional(),
  category: vine.string().trim().maxLength(50).optional(),
  capacity: vine.number().withoutDecimals().min(1).optional(),
  startsAt: vine.date({ formats: DT_FORMATS }),
  endsAt: vine.date({ formats: DT_FORMATS }).afterField('startsAt').optional(),
  salesOpensAt: vine.date({ formats: DT_FORMATS }).optional(),
  salesClosesAt: vine.date({ formats: DT_FORMATS }).afterField('salesOpensAt').optional(),
  isFeatured: vine.boolean().optional(),
  isPublic: vine.boolean().optional(),
})

export const updateEventSchema = vine.create({
  title: vine.string().trim().minLength(3).maxLength(200).optional(),
  description: vine.string().trim().maxLength(5000).optional(),
  coverImageUrl: vine.string().url().optional(),
  location: vine.string().trim().minLength(3).maxLength(300).optional(),
  city: vine.string().trim().minLength(2).maxLength(100).optional(),
  country: vine.string().trim().minLength(2).maxLength(100).optional(),
  address: vine.string().trim().maxLength(500).optional(),
  latitude: vine.number().min(-90).max(90).optional(),
  longitude: vine.number().min(-180).max(180).optional(),
  status: vine.enum(EVENT_STATUSES).optional(),
  category: vine.string().trim().maxLength(50).optional(),
  capacity: vine.number().withoutDecimals().min(1).optional(),
  startsAt: vine.date({ formats: DT_FORMATS }).optional(),
  endsAt: vine.date({ formats: DT_FORMATS }).optional(),
  salesOpensAt: vine.date({ formats: DT_FORMATS }).optional(),
  salesClosesAt: vine.date({ formats: DT_FORMATS }).optional(),
  isFeatured: vine.boolean().optional(),
  isPublic: vine.boolean().optional(),
})
