import { inject } from '@adonisjs/core'
import db from '@adonisjs/lucid/services/db'
import User from '#models/user'
import { VerificationService } from '#services/verification_service'
import { ErrorCode, ErrorCodeValue } from '#enums/error_codes'

export interface PasswordResetResult {
  ok: boolean
  message: string
  code?: ErrorCodeValue
}

@inject()
export class PasswordResetService {
  constructor(protected verificationService: VerificationService) { }

  /**
   * Send a password reset challenge to the user.
   */
  async sendResetCode(
    email?: string | null,
    phone?: string | null,
    method: 'otp' | 'token' = 'otp'
  ): Promise<{ ok: boolean; message: string }> {
    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    // Always return the same message to avoid user enumeration
    if (!user) {
      return {
        ok: true,
        message: `If that account exists, a reset ${method === 'otp' ? 'code' : 'link'} has been sent.`,
      }
    }

    await this.verificationService.sendChallenge(
      { userId: user.id, email, phone, name: user.fullName },
      'password_reset',
      method
    )

    return {
      ok: true,
      message: `If that account exists, a reset ${method === 'otp' ? 'code' : 'link'} has been sent.`,
    }
  }

  /**
   * Confirm password reset using OTP or token.
   */
  async confirmReset(
    email: string | undefined,
    phone: string | undefined,
    password: string,
    otp?: string,
    token?: string
  ): Promise<PasswordResetResult> {
    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    if (!user) {
      return {
        ok: false,
        message: 'Invalid email/phone or reset code.',
        code: ErrorCode.INVALID_TOKEN,
      }
    }

    // Determine the OTP type based on what was provided
    const type = token ? 'password_reset_token' : 'password_reset_otp'
    const value = (token || otp)!

    const record = await this.verificationService.verifyChallenge(type, value, user.id)

    if (!record) {
      return {
        ok: false,
        message: 'Invalid or expired reset code.',
        code: ErrorCode.INVALID_TOKEN,
      }
    }

    // Update password
    user.password = password
    await user.save()

    // Revoke all existing sessions — security measure
    await db.from('auth_access_tokens').where('tokenable_id', user.id).delete()

    return {
      ok: true,
      message: 'Password has been reset successfully.',
    }
  }
}
