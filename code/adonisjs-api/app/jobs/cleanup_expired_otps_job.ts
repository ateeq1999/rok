import { Job } from '@adonisjs/queue'
import type { JobOptions } from '@adonisjs/queue/types'
import { DateTime } from 'luxon'
import OtpCode from '#models/otp_code'

export default class CleanupExpiredOtpsJob extends Job<Record<string, never>> {
  static options: JobOptions = {
    queue: 'default',
    maxRetries: 2,
    timeout: '60s',
    removeOnComplete: { age: '1d' },
  }

  async execute() {
    const cutoff = DateTime.now().minus({ days: 1 }).toSQL()!
    const count = await OtpCode.query().where('expiresAt', '<', cutoff).delete()
    console.info(`[CleanupExpiredOtps] Deleted ${count} expired records`)
  }

  async failed(error: Error) {
    console.error('[CleanupExpiredOtps] Failed:', error.message)
  }
}
