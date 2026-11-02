import vine from '@vinejs/vine'

export const listPointLedgerSchema = vine.create({
  limit: vine.number().withoutDecimals().min(1).max(100).optional(),
  cursor: vine.string().optional(),
})

export const listRewardsSchema = vine.create({
  limit: vine.number().withoutDecimals().min(1).max(100).optional(),
})

export const adminCreateRewardSchema = vine.create({
  name: vine.string().trim().minLength(2).maxLength(200),
  description: vine.string().trim().maxLength(2000).optional(),
  pointsCost: vine.number().withoutDecimals().min(1),
  quantityTotal: vine.number().withoutDecimals().min(1).optional(),
  status: vine.enum(['active', 'inactive']).optional(),
  expiresAt: vine.date({ formats: { utc: true } }).optional(),
})

export const adminUpdateRewardSchema = vine.create({
  name: vine.string().trim().minLength(2).maxLength(200).optional(),
  description: vine.string().trim().maxLength(2000).optional(),
  pointsCost: vine.number().withoutDecimals().min(1).optional(),
  quantityTotal: vine.number().withoutDecimals().min(1).optional(),
  status: vine.enum(['active', 'inactive']).optional(),
  expiresAt: vine.date({ formats: { utc: true } }).optional(),
})
