import type { HttpContext } from '@adonisjs/core/http'
import { DateTime } from 'luxon'
import db from '@adonisjs/lucid/services/db'
import Order from '#models/order'
import OrderItem from '#models/order_item'
import Ticket from '#models/ticket'
import TicketType from '#models/ticket_type'
import OrderTransformer from '#transformers/order_transformer'
import { createOrderSchema } from '#validators/order'
import { created, success, notFound, conflict, unprocessable } from '#helpers/response'

const ORDER_EXPIRY_MINUTES = 15

export default class OrdersController {
  async store({ request, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const data = await request.validateUsing(createOrderSchema)

    if (data.idempotencyKey) {
      const existing = await Order.findBy('idempotencyKey', data.idempotencyKey)
      if (existing) {
        await existing.load('items')
        return created(response, OrderTransformer.transform(existing))
      }
    }

    const ticketTypeIds = data.items.map((i) => i.ticketTypeId)

    // Pre-validate max-per-order and existence (no lock needed — these don't change concurrently)
    const preCheck = await TicketType.query()
      .whereIn('id', ticketTypeIds)
      .where('eventId', data.eventId)
      .where('status', 'active')

    if (preCheck.length !== ticketTypeIds.length) {
      return unprocessable(response, 'One or more ticket types are invalid or unavailable')
    }

    const preMap = new Map(preCheck.map((tt) => [tt.id, tt]))
    for (const item of data.items) {
      const tt = preMap.get(item.ticketTypeId)!
      if (tt.maxPerOrder && item.quantity > tt.maxPerOrder) {
        return unprocessable(response, `Maximum ${tt.maxPerOrder} tickets per order for "${tt.name}"`)
      }
    }

    type LineItem = { ticketType: TicketType; quantity: number; unitPrice: number }
    let validationError: string | null = null
    let createdOrder: Order | null = null

    await db.transaction(async (trx) => {
      // Lock ticket type rows to prevent concurrent oversell
      const ticketTypes = await TicketType.query()
        .useTransaction(trx)
        .whereIn('id', ticketTypeIds)
        .where('eventId', data.eventId)
        .where('status', 'active')
        .forUpdate()

      const ttMap = new Map(ticketTypes.map((tt) => [tt.id, tt]))
      const lineItems: LineItem[] = []

      for (const item of data.items) {
        const tt = ttMap.get(item.ticketTypeId)!
        const available = tt.quantityTotal - (tt.quantitySold ?? 0)
        if (available < item.quantity) {
          validationError = `Not enough availability for "${tt.name}" (${available} remaining)`
          return
        }
      }

      let subtotal = 0
      for (const item of data.items) {
        const tt = ttMap.get(item.ticketTypeId)!
        const unitPrice = Number(tt.price ?? 0)
        subtotal += unitPrice * item.quantity
        lineItems.push({ ticketType: tt, quantity: item.quantity, unitPrice })
      }

      const order = new Order()
      order.useTransaction(trx)
      order.fill({
        userId: user.id,
        eventId: data.eventId,
        status: 'pending',
        subtotalAmount: subtotal,
        discountAmount: 0,
        totalAmount: subtotal,
        currency: ticketTypes[0].currency ?? 'TZS',
        idempotencyKey: data.idempotencyKey ?? null,
        expiresAt: DateTime.now().plus({ minutes: ORDER_EXPIRY_MINUTES }),
      })
      await order.save()

      const items: OrderItem[] = []
      for (const li of lineItems) {
        const item = new OrderItem()
        item.useTransaction(trx)
        item.fill({
          orderId: order.id,
          ticketTypeId: li.ticketType.id,
          quantity: li.quantity,
          unitPrice: li.unitPrice,
        })
        await item.save()
        items.push(item)
      }

      order.$setRelated('items', items)
      createdOrder = order
    })

    if (validationError) {
      return unprocessable(response, validationError)
    }

    return created(response, OrderTransformer.transform(createdOrder!))
  }

  async issueTickets({ params, response }: HttpContext) {
    const order = await Order.findOrFail(params.id)
    await order.load('items')

    if (order.status !== 'paid') {
      return unprocessable(response, 'Order must be paid to issue tickets')
    }

    const issuedTickets = []

    for (const item of order.items) {
      for (let i = 0; i < item.quantity; i++) {
        const ticket = await Ticket.create({
          orderId: order.id,
          eventId: order.eventId,
          userId: order.userId,
          ticketTypeId: item.ticketTypeId,
          status: 'issued',
        })

        await ticket.generateSecret()
        issuedTickets.push(ticket)
      }
    }

    order.status = 'issued'
    await order.save()

    return success(response, { issued: issuedTickets.length })
  }

  async show({ params, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()

    const order = await Order.query()
      .where('id', params.id)
      .where('userId', user.id)
      .preload('items')
      .first()

    if (!order) {
      return notFound(response, 'Order not found')
    }

    return success(response, OrderTransformer.transform(order))
  }

  async cancel({ params, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()

    const order = await Order.query()
      .where('id', params.id)
      .where('userId', user.id)
      .first()

    if (!order) {
      return notFound(response, 'Order not found')
    }

    if (order.status === 'paid' || order.status === 'issued') {
      return conflict(response, 'Cannot cancel order that is already paid or issued')
    }

    order.status = 'cancelled'
    await order.save()

    return success(response, { id: order.id, status: order.status })
  }
}
