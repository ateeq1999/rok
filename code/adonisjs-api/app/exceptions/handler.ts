import app from '@adonisjs/core/services/app'
import { type HttpContext, ExceptionHandler, errors as httpErrors } from '@adonisjs/core/http'
import { errors as authErrors } from '@adonisjs/auth'
import { errors as vinErrors } from '@vinejs/vine'
import { errors as lucidErrors } from '@adonisjs/lucid'
import { errors as limiterErrors } from '@adonisjs/limiter'
import ApiResponse from '#services/api_response'
import { ErrorCode } from '#enums/error_codes'

export default class HttpExceptionHandler extends ExceptionHandler {
  protected debug = !app.inProduction

  async handle(error: unknown, ctx: HttpContext) {
    if (error instanceof vinErrors.E_VALIDATION_ERROR) {
      return ApiResponse.unprocessable(
        ctx.response,
        'Validation failed',
        error.messages,
        ErrorCode.VALIDATION_FAILED
      )
    }

    if (error instanceof authErrors.E_UNAUTHORIZED_ACCESS) {
      return ApiResponse.unauthorized(
        ctx.response,
        'Authentication required',
        undefined,
        ErrorCode.UNAUTHORIZED
      )
    }

    if (error instanceof authErrors.E_INVALID_CREDENTIALS) {
      return ApiResponse.unauthorized(
        ctx.response,
        'Invalid credentials',
        undefined,
        ErrorCode.INVALID_CREDENTIALS
      )
    }

    if (error instanceof limiterErrors.E_TOO_MANY_REQUESTS) {
      const headers = error.getDefaultHeaders()
      Object.keys(headers).forEach((h) => ctx.response.header(h, headers[h]))
      return ApiResponse.error(
        ctx.response,
        error.getResponseMessage(ctx) as string,
        null,
        429,
        ErrorCode.RATE_LIMITED
      )
    }

    if (error instanceof lucidErrors.E_ROW_NOT_FOUND) {
      return ApiResponse.notFound(ctx.response, 'Resource not found', ErrorCode.NOT_FOUND)
    }

    if (error instanceof httpErrors.E_ROUTE_NOT_FOUND) {
      return ApiResponse.notFound(ctx.response, 'Route not found', ErrorCode.NOT_FOUND)
    }

    return super.handle(error, ctx)
  }

  async report(error: unknown, ctx: HttpContext) {
    return super.report(error, ctx)
  }
}
