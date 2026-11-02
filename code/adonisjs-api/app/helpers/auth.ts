import env from '#start/env'
import type { HttpContext } from '@adonisjs/core/http'
import User from '#models/user'
import UserTransformer from '#transformers/user_transformer'

export const cookieOptions = () => ({
  httpOnly: env.get('COOKIE_HTTP_ONLY', true),
  secure: env.get('COOKIE_SECURE', false),
  sameSite: env.get('COOKIE_SAME_SITE', 'lax') as 'lax' | 'strict' | 'none',
  path: env.get('COOKIE_PATH', '/'),
  domain: env.get('COOKIE_DOMAIN', ''),
  maxAge: env.get('COOKIE_MAX_AGE', '2h'),
})

export const COOKIE_NAME = env.get('COOKIE_NAME', 'jioni_token')

export type TokenStyle = 'cookies' | 'bearer'

export function getTokenStyle(request: HttpContext['request']): TokenStyle {
  const { token } = request.qs()
  return token === 'bearer' ? 'bearer' : 'cookies'
}


/**
 * Send a token response using the appropriate style (bearer token or
 * encrypted cookie). Keeps controller methods short and consistent.
 */
export function respondWithAuth(ctx: HttpContext, token: string, user: User) {
  const { request, response, serialize } = ctx
  const tokenStyle = getTokenStyle(request)

  if (tokenStyle === 'bearer') {
    return serialize({ ok: true, user: UserTransformer.transform(user), token })
  }

  response.encryptedCookie(COOKIE_NAME, token, cookieOptions())
  return serialize({ ok: true, user: UserTransformer.transform(user) })
}
