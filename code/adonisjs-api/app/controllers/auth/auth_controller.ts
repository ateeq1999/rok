import type { HttpContext } from '@adonisjs/core/http'
import { inject } from '@adonisjs/core'
import { AuthService } from '#services/auth_service'
import {
  registerWithEmailValidator,
  registerWithPhoneValidator,
  loginWithPasswordValidator,
} from '#validators/user'
import { ErrorCode } from '#enums/error_codes'
import UserTransformer from '#transformers/user_transformer'
import { COOKIE_NAME, getTokenStyle, respondWithAuth } from '#helpers/auth'

@inject()
export default class AuthController {
  constructor(protected authService: AuthService) { }

  /**
   * Register with email + password.
   */
  async registerWithEmail(ctx: HttpContext) {
    const payload = await ctx.request.validateUsing(registerWithEmailValidator)
    const { token, user } = await this.authService.register(payload)
    return respondWithAuth(ctx, token, user)
  }

  /**
   * Register with phone + password.
   */
  async registerWithPhone(ctx: HttpContext) {
    const payload = await ctx.request.validateUsing(registerWithPhoneValidator)

    const { token, user } = await this.authService.register(payload)
    return respondWithAuth(ctx, token, user)
  }

  /**
   * Login with email/phone + password.
   */
  async login(ctx: HttpContext) {
    const payload = await ctx.request.validateUsing(loginWithPasswordValidator)

    try {
      const { token, user } = await this.authService.login(payload)
      return respondWithAuth(ctx, token, user)
    } catch {
      return ctx.serialize({
        ok: false,
        message: 'Invalid credentials.',
        code: ErrorCode.INVALID_CREDENTIALS,
      })
    }
  }

  /**
   * Return the currently authenticated user.
   */
  async me({ auth, serialize }: HttpContext) {
    const user = auth.getUserOrFail()
    return serialize({
      ok: true,
      user: UserTransformer.transform(user),
    })
  }

  /**
   * Refresh the access token.
   */
  async refresh(ctx: HttpContext) {
    const user = ctx.auth.getUserOrFail()
    const { token } = await this.authService.refreshToken(user)
    return respondWithAuth(ctx, token, user)
  }

  /**
   * Logout — revoke current token.
   */
  async logout(ctx: HttpContext) {
    const user = ctx.auth.getUserOrFail()
    await this.authService.logout(user)

    const tokenStyle = getTokenStyle(ctx.request)
    if (tokenStyle === 'bearer') {
      return ctx.serialize({ ok: true, message: 'Logged out successfully.' })
    }

    ctx.response.clearCookie(COOKIE_NAME)
    ctx.response.status(204).send('')
  }
}
