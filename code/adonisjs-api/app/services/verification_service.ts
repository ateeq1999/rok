import { inject } from '@adonisjs/core'
import { OtpService } from '#services/otp_service'
import SendEmailJob from '#jobs/send_email_job'
import SendSmsJob from '#jobs/send_sms_job'
import type { OtpType } from '#models/otp_code'

interface SendVerificationParams {
  userId: string
  email?: string | null
  phone?: string | null
  name?: string | null
}

type VerificationChannel = 'email' | 'phone'

@inject()
export class VerificationService {
  constructor(protected otpService: OtpService) { }

  private getChannel(params: SendVerificationParams): VerificationChannel {
    if (params.email) return 'email'
    if (params.phone) return 'phone'
    throw new Error('Either email or phone must be provided')
  }

  private resolveType(
    channel: VerificationChannel,
    purpose: 'verify' | 'password_reset' | 'otp_login',
    method?: 'otp' | 'token'
  ): OtpType {
    const map: Record<VerificationChannel, Record<string, OtpType>> = {
      email: {
        verify: 'email_verify',
        password_reset_otp: 'password_reset_otp',
        password_reset_token: 'password_reset_token',
        otp_login: 'otp_login',
      },
      phone: {
        verify: 'phone_verify',
        password_reset_otp: 'password_reset_otp',
        password_reset_token: 'password_reset_token',
        otp_login: 'otp_login',
      },
    }

    const key = purpose === 'password_reset' ? `password_reset_${method ?? 'otp'}` : purpose
    return map[channel][key]
  }

  async sendChallenge(
    params: SendVerificationParams,
    purpose: 'verify' | 'password_reset' | 'otp_login',
    method?: 'otp' | 'token'
  ): Promise<{ code: string; type: OtpType }> {
    const channel = this.getChannel(params)
    const type = this.resolveType(channel, purpose, method)

    const isToken = type.endsWith('_token')
    const code = isToken
      ? await this.otpService.generateToken()
      : await this.otpService.generateOtp()

    await this.otpService.create(params.userId, type, code)

    const name = params.name ?? null

    if (channel === 'email') {
      const email = params.email!
      if (type === 'email_verify') {
        await SendEmailJob.dispatch({ type: 'email_verify', email, name, token: code }).run()
      } else if (type === 'password_reset_otp') {
        await SendEmailJob.dispatch({ type: 'password_reset_otp', email, name, otp: code }).run()
      } else if (type === 'password_reset_token') {
        await SendEmailJob.dispatch({ type: 'password_reset_token', email, name, token: code }).run()
      } else if (type === 'otp_login') {
        await SendEmailJob.dispatch({ type: 'otp_login', email, name, otp: code }).run()
      }
    } else {
      const phone = params.phone!
      if (type === 'phone_verify') {
        await SendSmsJob.dispatch({ type: 'phone_verify', phone, otp: code }).run()
      } else if (type === 'password_reset_otp') {
        await SendSmsJob.dispatch({ type: 'password_reset_otp', phone, otp: code }).run()
      } else if (type === 'otp_login') {
        await SendSmsJob.dispatch({ type: 'otp_login', phone, otp: code }).run()
      }
    }

    return { code, type }
  }

  async verifyChallenge(
    type: OtpType,
    value: string,
    userId?: string
  ) {
    return this.otpService.verify(type, value, userId)
  }
}
