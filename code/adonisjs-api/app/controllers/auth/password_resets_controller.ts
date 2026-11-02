import type { HttpContext } from '@adonisjs/core/http'
import { inject } from '@adonisjs/core'
import { PasswordResetService } from '#services/password_reset_service'
import { passwordResetRequestValidator, passwordResetConfirmValidator } from '#validators/user'

@inject()
export default class PasswordResetsController {
  constructor(protected passwordResetService: PasswordResetService) { }

  /**
   * Request a password reset code or link.
   * Supports: email otp, email token, phone otp.
   */
  async request({ request, serialize }: HttpContext) {
    const { email, phone, method } = await request.validateUsing(passwordResetRequestValidator)
    const result = await this.passwordResetService.sendResetCode(email, phone, method ?? 'otp')
    return serialize(result)
  }

  /**
   * Confirm password reset with OTP or token.
   */
  async confirm({ request, serialize }: HttpContext) {
    const data = await request.validateUsing(passwordResetConfirmValidator)
    const result = await this.passwordResetService.confirmReset(
      data.email,
      data.phone,
      data.password,
      data.otp,
      data.token
    )
    return serialize(result)
  }
}
