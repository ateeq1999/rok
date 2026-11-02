import type { HttpContext } from '@adonisjs/core/http'
import { inject } from '@adonisjs/core'
import User from '#models/user'
import { VerificationService } from '#services/verification_service'
import { AuthService } from '#services/auth_service'
import { verifyAccountValidator, resendVerificationValidator } from '#validators/user'
import { ErrorCode } from '#enums/error_codes'

@inject()
export default class VerificationController {
  constructor(
    protected verificationService: VerificationService,
    protected authService: AuthService
  ) { }

  /**
   * Verify an email or phone with an OTP or token.
   */
  async verify({ request, serialize }: HttpContext) {
    const { email, phone, otp, token } = await request.validateUsing(verifyAccountValidator)

    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    if (!user) {
      return serialize({
        ok: false,
        message: 'User not found.',
        code: ErrorCode.USER_NOT_FOUND,
      })
    }

    const type = email ? 'email_verify' : 'phone_verify'
    const value = (token || otp)!
    const success = await this.authService.verifyAccount(user, type, value)

    if (!success) {
      return serialize({
        ok: false,
        message: 'Invalid or expired verification code.',
        code: ErrorCode.INVALID_TOKEN,
      })
    }

    return serialize({
      ok: true,
      message: `${email ? 'Email' : 'Phone'} verified successfully.`,
    })
  }

  /**
   * Resend a verification challenge.
   */
  async resend({ request, serialize }: HttpContext) {
    const { email, phone } = await request.validateUsing(resendVerificationValidator)

    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    if (user) {
      await this.verificationService.sendChallenge(
        { userId: user.id, email, phone, name: user.fullName },
        'verify',
        email ? 'token' : 'otp'
      )
    }

    return serialize({
      ok: true,
      message: 'If that account exists, a new verification code has been sent.',
    })
  }
}
