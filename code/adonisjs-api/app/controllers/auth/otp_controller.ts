import type { HttpContext } from '@adonisjs/core/http'
import { inject } from '@adonisjs/core'
import User from '#models/user'
import { VerificationService } from '#services/verification_service'
import { AuthService } from '#services/auth_service'
import { otpLoginRequestValidator, otpLoginVerifyValidator } from '#validators/user'
import { ErrorCode } from '#enums/error_codes'
import { respondWithAuth } from '#helpers/auth'

@inject()
export default class OtpController {
  constructor(
    protected verificationService: VerificationService,
    protected authService: AuthService
  ) { }

  /**
   * Request an OTP for login (email or phone).
   */
  async request({ request, serialize }: HttpContext) {
    const { email, phone } = await request.validateUsing(otpLoginRequestValidator)
    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    if (user) {
      await this.verificationService.sendChallenge(
        { userId: user.id, email, phone, name: user.fullName },
        'otp_login',
        'otp'
      )
    }

    // Always return the same message
    return serialize({
      ok: true,
      message: 'If that account exists, a login code has been sent.',
    })
  }

  /**
   * Verify OTP and authenticate.
   */
  async verify(ctx: HttpContext) {
    const { email, phone, otp } = await ctx.request.validateUsing(otpLoginVerifyValidator)
    const where = email ? { email } : { phone: phone! }
    const user = await User.findBy(where)

    if (!user) {
      return ctx.serialize({
        ok: false,
        message: 'Invalid credentials.',
        code: ErrorCode.INVALID_CREDENTIALS,
      })
    }

    const record = await this.verificationService.verifyChallenge('otp_login', otp, user.id)

    if (!record) {
      return ctx.serialize({
        ok: false,
        message: 'Invalid or expired login code.',
        code: ErrorCode.INVALID_TOKEN,
      })
    }

    const token = await this.authService.createTokenForUser(user)
    return respondWithAuth(ctx, token, user)
  }
}
