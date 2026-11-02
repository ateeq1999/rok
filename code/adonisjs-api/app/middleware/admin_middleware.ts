import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'
import ApiResponse from '#services/api_response'
import { UserRole } from '#enums/index'

export default class AdminMiddleware {
  async handle(ctx: HttpContext, next: NextFn) {
    const user = ctx.auth.getUserOrFail()

    if (user.role !== UserRole.ADMIN) {
      return ApiResponse.forbidden(ctx.response, 'Admin access required')
    }

    return next()
  }
}
