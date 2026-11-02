import vine from '@vinejs/vine'
import { IMAGE_EXTS, MEDIA_EXTS } from '#helpers/formats'
import { CONTENT_TIERS, CONTENT_TYPES } from '#enums/index'

export const listContentSchema = vine.create({
  tier: vine.string().trim().optional(),
  type: vine.string().trim().optional(),
  limit: vine.number().withoutDecimals().min(1).max(100).optional(),
  cursor: vine.string().optional(),
})

export const adminListContentSchema = vine.compile(
  vine.object({
    page: vine.number().withoutDecimals().min(1).optional(),
    perPage: vine.number().withoutDecimals().min(1).max(100).optional(),
    type: vine.string().trim().optional(),
    tier: vine.string().trim().optional(),
  })
)

export const createContentSchema = vine.create({
  title: vine.string().trim().minLength(3).maxLength(200),
  type: vine.enum(CONTENT_TYPES),
  tier: vine.enum(CONTENT_TIERS),
  description: vine.string().trim().maxLength(2000).optional(),
  body: vine.string().trim().optional(),
  thumbnailUrl: vine.string().url().optional(),
  videoUrl: vine.string().url().optional(),
  durationSeconds: vine.number().withoutDecimals().min(1).optional(),
})

export const updateContentSchema = vine.create({
  title: vine.string().trim().minLength(3).maxLength(200).optional(),
  type: vine.enum(CONTENT_TYPES).optional(),
  tier: vine.enum(CONTENT_TIERS).optional(),
  description: vine.string().trim().maxLength(2000).optional(),
  body: vine.string().trim().optional(),
  thumbnailUrl: vine.string().url().optional(),
  videoUrl: vine.string().url().optional(),
  durationSeconds: vine.number().withoutDecimals().min(1).optional(),
})

export const uploadContentMediaSchema = vine.create({
  uploadType: vine.enum(['thumbnail', 'media'] as const).optional(),
  file: vine.file({
    size: '10mb',
    extnames: [...IMAGE_EXTS, ...MEDIA_EXTS],
  }),
})
