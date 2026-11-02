import { Job } from '@adonisjs/queue'
import type { JobOptions } from '@adonisjs/queue/types'
import { DateTime } from 'luxon'
import Order from '#models/order'

export default class CleanupExpiredOrdersJob extends Job<Record<string, never>> {
  static options: JobOptions = {
    queue: 'default',
    maxRetries: 2,
    timeout: '60s',
    removeOnComplete: { age: '1d' },
  }

  async execute() {
    const count = await Order.query()
      .where('status', 'pending')
      .where('expiresAt', '<', DateTime.now().toSQL()!)
      .update({ status: 'cancelled' })
    console.info(`[CleanupExpiredOrders] Cancelled ${count} expired pending orders`)
  }

  async failed(error: Error) {
    console.error('[CleanupExpiredOrders] Failed:', error.message)
  }
}
