import type { HttpContext } from '@adonisjs/core/http'
import { inject } from '@adonisjs/core'
import { DateTime } from 'luxon'
import User from '#models/user'
import { OtpService } from '#services/otp_service'
import { VerificationService } from '#services/verification_service'
import UserTransformer from '#transformers/user_transformer'
import { ErrorCode } from '#enums/error_codes'

@inject()
export default class EmailVerificationController {
  constructor(
    protected otpService: OtpService,
    protected verificationService: VerificationService
  ) { }

  async verify({ params, serialize }: HttpContext) {
    const record = await this.otpService.verify('email_verify', params.token)

    if (!record) {
      return serialize({
        ok: false,
        message: 'Invalid or expired verification link',
        code: ErrorCode.INVALID_TOKEN,
      })
    }

    const user = await User.findOrFail(record.userId)
    user.emailVerifiedAt = DateTime.now()
    user.emailVerified = true
    await user.save()

    return serialize({
      ok: true,
      verified: true,
      user: UserTransformer.transform(user),
    })
  }

  async resend({ auth, serialize }: HttpContext) {
    const user = auth.getUserOrFail()

    if (user.emailVerified) {
      return serialize({
        ok: false,
        message: 'Email is already verified.',
        code: ErrorCode.EMAIL_ALREADY_VERIFIED,
      })
    }

    await this.verificationService.sendChallenge(
      { userId: user.id, email: user.email, name: user.fullName },
      'verify',
      'token'
    )

    return serialize({
      ok: true,
      message: 'Verification email resent.',
    })
  }
}
