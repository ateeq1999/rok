import app from '@adonisjs/core/services/app'
import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'

const MUTATING_METHODS = new Set(['POST', 'PUT', 'PATCH', 'DELETE'])

// Routes exempt from origin checks (webhooks, health, public callbacks)
const EXEMPT_PREFIXES = ['/health', '/api/v1/auth/otp/', '/api/v1/auth/email/']

function getAllowedOrigins(): string[] {
  const raw = process.env.CORS_ORIGIN ?? process.env.APP_URL ?? ''
  return raw
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean)
}

export default class OriginCheckMiddleware {
  async handle({ request, response, serialize }: HttpContext, next: NextFn) {
    // Only enforce in production and for state-changing methods
    if (app.inDev || !MUTATING_METHODS.has(request.method())) {
      return next()
    }

    const url = request.url()
    if (EXEMPT_PREFIXES.some((p) => url.startsWith(p))) {
      return next()
    }

    const origin = request.header('origin') ?? request.header('referer')

    if (!origin) {
      // Non-browser clients (mobile, server-to-server) have no Origin — allow
      return next()
    }

    const allowed = getAllowedOrigins()
    const originHost = (() => {
      try {
        return new URL(origin).origin
      } catch {
        return null
      }
    })()

    if (!originHost || !allowed.includes(originHost)) {
      response.status(403)
      return serialize({ ok: false, message: 'Forbidden origin' })
    }

    return next()
  }
}
