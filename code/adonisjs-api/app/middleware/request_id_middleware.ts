import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'
import { ulid } from '@std/ulid'

declare module '@adonisjs/core/http' {
  interface HttpContext {
    requestId: string
  }
}

export default class RequestIdMiddleware {
  async handle(ctx: HttpContext, next: NextFn) {
    ctx.requestId = `req_${ulid()}`
    ctx.response.header('X-Request-Id', ctx.requestId)

    await next()

    // Inject into response body meta when using our envelope format
    const body = ctx.response.getBody?.() as Record<string, unknown> | undefined
    if (body && typeof body === 'object' && !Array.isArray(body)) {
      const meta = body['meta']
      if (meta && typeof meta === 'object' && !Array.isArray(meta)) {
        ; (meta as Record<string, unknown>)['requestId'] = ctx.requestId
      }
    }
  }
}
