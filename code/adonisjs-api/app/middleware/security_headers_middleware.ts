import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'

export default class SecurityHeadersMiddleware {
  async handle({ response }: HttpContext, next: NextFn) {
    await next()

    response.header('Referrer-Policy', 'strict-origin-when-cross-origin')
    response.header(
      'Permissions-Policy',
      'camera=(), microphone=(), geolocation=(), payment=()'
    )
    response.header('X-Permitted-Cross-Domain-Policies', 'none')
  }
}
