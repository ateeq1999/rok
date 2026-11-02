import { Job, exponentialBackoff } from '@adonisjs/queue'
import type { JobOptions } from '@adonisjs/queue/types'
import mail from '@adonisjs/mail/services/main'
import { EmailVerificationMail } from '#mails/email_verification_mail'
import { PasswordResetMail } from '#mails/password_reset_mail'
import { WelcomeMail } from '#mails/welcome_mail'
import { OrderConfirmationMail } from '#mails/order_confirmation_mail'
import { OtpLoginMail } from '#mails/otp_login_mail'

export type EmailJobType =
  | { type: 'email_verify'; email: string; name: string | null; token: string }
  | { type: 'password_reset_otp'; email: string; name: string | null; otp: string }
  | { type: 'password_reset_token'; email: string; name: string | null; token: string }
  | { type: 'otp_login'; email: string; name: string | null; otp: string }
  | { type: 'welcome'; email: string; name: string | null }
  | {
      type: 'order_confirmation'
      email: string
      name: string | null
      orderId: string
      eventTitle: string
      ticketCount: number
      total: number
    }

export default class SendEmailJob extends Job<EmailJobType> {
  static options: JobOptions = {
    queue: 'emails',
    maxRetries: 3,
    retry: { backoff: exponentialBackoff({ baseDelay: '5s', maxDelay: '2m' }) },
    timeout: '30s',
    removeOnComplete: { age: '7d', count: 500 },
    removeOnFail: false,
  }

  async execute() {
    const { payload } = this

    switch (payload.type) {
      case 'email_verify':
        await mail.send(new EmailVerificationMail(payload.email, payload.name, payload.token))
        break
      case 'password_reset_otp':
        await mail.send(new PasswordResetMail(payload.email, payload.name, payload.otp))
        break
      case 'password_reset_token':
        await mail.send(new PasswordResetMail(payload.email, payload.name, payload.token))
        break
      case 'otp_login':
        await mail.send(new OtpLoginMail(payload.email, payload.name, payload.otp))
        break
      case 'welcome':
        await mail.send(new WelcomeMail(payload.email, payload.name))
        break
      case 'order_confirmation':
        await mail.send(
          new OrderConfirmationMail(
            payload.email,
            payload.name,
            payload.orderId,
            payload.eventTitle,
            payload.ticketCount,
            payload.total
          )
        )
        break
    }
  }

  async failed(error: Error) {
    console.error('[SendEmailJob] failed', { type: this.payload.type, error: error.message })
  }
}
