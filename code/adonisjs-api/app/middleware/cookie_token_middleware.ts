import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'
import { COOKIE_NAME } from '#helpers/auth'

export default class CookieTokenMiddleware {
  async handle({ request }: HttpContext, next: NextFn) {
    if (!request.header('Authorization')) {
      const token = request.encryptedCookie(COOKIE_NAME)
      if (token) {
        request.request.headers['authorization'] = `Bearer ${token}`
      }
    }
    return next()
  }
}
