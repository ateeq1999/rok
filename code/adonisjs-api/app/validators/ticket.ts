import vine from '@vinejs/vine'

export const listMyTicketsSchema = vine.create(
  vine.object({
    cursor: vine.string().optional(),
    limit: vine.number().withoutDecimals().min(1).max(50).optional(),
  })
)

export const scanTicketSchema = vine.create(
  vine.object({
    secret: vine.string().fixedLength(64),
  })
)
