import vine from '@vinejs/vine'

export const createOrderSchema = vine.create({
  eventId: vine.string(),
  items: vine
    .array(
      vine.object({
        ticketTypeId: vine.string(),
        quantity: vine.number().withoutDecimals().min(1).max(20),
      })
    )
    .minLength(1),
  paymentMethod: vine.enum(['mpesa', 'card', 'stripe']).optional(),
  mpesaPhone: vine.string().optional(),
  idempotencyKey: vine.string().optional(),
})
