import { BaseTransformer } from '@adonisjs/core/transformers'
import type Order from '#models/order'
import type OrderItem from '#models/order_item'

export type OrderStatus = 'pending_payment' | 'paid' | 'cancelled' | 'refunded' | 'expired'

class OrderItemTransformer extends BaseTransformer<OrderItem> {
  toObject() {
    return {
      ...this.pick(this.resource, ['id', 'ticketTypeId', 'quantity', 'unitPrice']),
      subtotal: this.resource.quantity * this.resource.unitPrice,
    }
  }
}

export default class OrderTransformer extends BaseTransformer<Order> {
  toObject() {
    return {
      ...this.pick(this.resource, [
        'id',
        'eventId',
        'status',
        'subtotalAmount',
        'totalAmount',
        'currency',
      ]),
      discountAmount: this.resource.discountAmount ?? 0,
      paidAt: this.resource.paidAt?.toISO() ?? null,
      expiresAt: this.resource.expiresAt.toISO()!,
      createdAt: this.resource.createdAt.toISO()!,
      items: (this.resource.items ?? []).map((item) => new OrderItemTransformer(item).toObject()),
    }
  }
}
