import type { HttpResponse } from '@adonisjs/core/http'

export interface FieldError {
  message: string
  field?: string   // VineJS validation errors use "field"
  path?: string    // custom field errors may use "path"
  value?: unknown
}

export default class ApiResponse {
  // ─── Success Responses ───────────────────────────────────────────

  static ok<T>(response: HttpResponse, data: T, meta?: unknown) {
    return response.status(200).json({
      ok: true,
      data,
      meta,
    })
  }

  static created<T>(response: HttpResponse, data: T, meta?: unknown) {
    return response.status(201).json({
      ok: true,
      data,
      meta,
    })
  }

  static noContent(response: HttpResponse) {
    return response.status(204).send('')
  }

  // ─── Error Responses ───────────────────────────────────────────

  static error(
    response: HttpResponse,
    message: string,
    errors: FieldError[] | null = null,
    status = 400,
    code?: string
  ) {
    return response.status(status).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        ...(errors !== null && { errors }),
      },
    })
  }

  static badRequest(
    response: HttpResponse,
    message: string,
    errors?: FieldError[],
    code?: string
  ) {
    return response.status(400).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        errors: errors ?? [{ message }],
      },
    })
  }

  static unauthorized(
    response: HttpResponse,
    message = 'Unauthorized',
    errors?: FieldError[],
    code?: string
  ) {
    return response.status(401).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        errors: errors ?? [{ message }],
      },
    })
  }

  static forbidden(response: HttpResponse, message = 'Forbidden', code?: string) {
    return this.error(response, message, null, 403, code)
  }

  static notFound(response: HttpResponse, message = 'Resource not found', code?: string) {
    return this.error(response, message, null, 404, code)
  }

  static conflict(
    response: HttpResponse,
    message: string,
    errors?: FieldError[],
    code?: string
  ) {
    return response.status(409).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        errors: errors ?? [{ message }],
      },
    })
  }

  static unprocessable(
    response: HttpResponse,
    message: string,
    errors?: FieldError[],
    code?: string
  ) {
    return response.status(422).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        errors: errors ?? [{ message }],
      },
    })
  }

  static serverError(
    response: HttpResponse,
    message = 'Internal server error',
    errors?: FieldError[],
    code?: string
  ) {
    return response.status(500).json({
      ok: false,
      error: {
        message,
        ...(code !== undefined && { code }),
        errors: errors ?? [{ message }],
      },
    })
  }
}
