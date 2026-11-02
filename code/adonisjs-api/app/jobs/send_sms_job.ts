import { Job } from '@adonisjs/queue'
import type { JobOptions } from '@adonisjs/queue/types'

export type SmsJobType =
  | { type: 'phone_verify'; phone: string; otp: string }
  | { type: 'password_reset_otp'; phone: string; otp: string }
  | { type: 'otp_login'; phone: string; otp: string }

export default class SendSmsJob extends Job<SmsJobType> {
  static options: JobOptions = {
    queue: 'sms',
    maxRetries: 3,
    timeout: '30s',
    removeOnComplete: { age: '7d', count: 500 },
    removeOnFail: false,
  }

  async execute() {
    const { payload } = this
    // TODO: integrate an SMS provider (e.g. Africa's Talking, Twilio)
    console.log('[SendSmsJob] would send SMS', { type: payload.type, phone: payload.phone })
  }

  async failed(error: Error) {
    console.error('[SendSmsJob] failed', { type: this.payload.type, error: error.message })
  }
}
