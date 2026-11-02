import type { HttpResponse } from '@adonisjs/core/http'

export interface FieldError {
  message: string
  field?: string
  path?: string
  value?: unknown
}

// ─── Success Object Builder (no response sent) ───────────────────
export function ok<T>(data: T, meta?: unknown) {
  return { ok: true, data, meta } as const
}

export function success<T>(response: HttpResponse, data: T, meta?: unknown) {
  return response.status(200).json({ ok: true, data, meta })
}

// ─── Response Senders ─────────────────────────────────────────────
export function created<T>(response: HttpResponse, data: T, meta?: unknown) {
  return response.status(201).json({ ok: true, data, meta })
}

export function noContent(response: HttpResponse) {
  return response.status(204).send('')
}

export function paginated<T>(response: HttpResponse, data: T, meta: any) {
  return response.status(200).json({ ok: true, data, meta })
}

export function cursor<T>(response: HttpResponse, data: T, nextCursor: string | null) {
  return response.status(200).json({ ok: true, data, meta: { nextCursor } })
}

// ─── Generic Error Sender ─────────────────────────────────────────
export function sendError(
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

// ─── Specific Error Helpers ───────────────────────────────────────
export function badRequest(
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

export function unauthorized(
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

export function forbidden(response: HttpResponse, message = 'Forbidden', code?: string) {
  return sendError(response, message, null, 403, code)
}

export function notFound(response: HttpResponse, message = 'Resource not found', code?: string) {
  return sendError(response, message, null, 404, code)
}

export function conflict(
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

export function unprocessable(
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

export function serverError(
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
