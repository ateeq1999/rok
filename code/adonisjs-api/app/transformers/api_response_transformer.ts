import { BaseTransformer } from '@adonisjs/core/transformers'

export interface CursorPayload {
  id: string
  ts: string
}

export interface BaseMeta {
  timestamp: string
  requestId?: string
}

export interface CursorMeta extends BaseMeta {
  nextCursor: string | null
}

export interface OffsetMeta extends BaseMeta {
  total: number
  page: number
  pageSize: number
  lastPage: number
}

export interface ApiError {
  code: string
  message: string
  details?: unknown
}

export interface OkResponse<T, M extends BaseMeta = BaseMeta> {
  ok: true
  data: T
  meta: M
}

export interface ErrorResponse {
  ok: false
  error: ApiError
  meta: BaseMeta
}

export type ApiResponse<T, M extends BaseMeta = BaseMeta> =
  | OkResponse<T, M>
  | ErrorResponse

export default class ApiResponseTransformer extends BaseTransformer<any> {
  /**
   * Base metadata
   */
  static baseMeta(): BaseMeta {
    return {
      timestamp: new Date().toISOString(),
    }
  }

  /**
   * Standard success response
   */
  static ok<T>(data: T): OkResponse<T> {
    return {
      ok: true,
      data,
      meta: this.baseMeta(),
    }
  }

  /**
   * Cursor pagination response
   */
  static cursor<T>(
    data: T,
    nextCursor: string | null
  ): OkResponse<T, CursorMeta> {
    return {
      ok: true,
      data,
      meta: {
        ...this.baseMeta(),
        nextCursor,
      },
    }
  }

  /**
   * Offset pagination response
   */
  static paginated<T>(
    data: T,
    page: number,
    pageSize: number,
    total: number
  ): OkResponse<T, OffsetMeta> {
    return {
      ok: true,
      data,
      meta: {
        ...this.baseMeta(),
        total,
        page,
        pageSize,
        lastPage: Math.ceil(total / pageSize),
      },
    }
  }

  /**
   * Error response
   */
  static fail(
    code: string,
    message: string,
    details?: unknown
  ): ErrorResponse {
    return {
      ok: false,
      error: {
        code,
        message,
        ...(details !== undefined ? { details } : {}),
      },
      meta: this.baseMeta(),
    }
  }

  /**
   * Encode cursor
   */
  static encodeCursor(id: string, ts: string): string {
    return Buffer.from(
      JSON.stringify({
        id,
        ts,
      } satisfies CursorPayload)
    ).toString('base64url')
  }

  /**
   * Decode cursor
   */
  static decodeCursor(raw: string): CursorPayload | null {
    try {
      const parsed = JSON.parse(
        Buffer.from(raw, 'base64url').toString('utf8')
      )

      if (
        typeof parsed?.id === 'string' &&
        typeof parsed?.ts === 'string'
      ) {
        return parsed as CursorPayload
      }

      return null
    } catch {
      return null
    }
  }

  /**
   * Apply keyset pagination condition
   */
  static applyCursorCondition(
    query: {
      where: (callback: (q: any) => void) => any
    },
    cursor: CursorPayload,
    sortField: string,
    sortOrder: 'asc' | 'desc' = 'asc'
  ): void {
    const op = sortOrder === 'asc' ? '>' : '<'

    query.where((q) => {
      q.where(sortField, op, cursor.ts).orWhere((q2: any) => {
        q2.where(sortField, cursor.ts).andWhere('id', op, cursor.id)
      })
    })
  }

  /**
   * Compute next cursor from over-fetched rows
   */
  static paginateCursor<T>(
    results: T[],
    limit: number,
    getTs: (item: T) => string,
    getId: (item: T) => string
  ): {
    items: T[]
    nextCursor: string | null
    hasNextPage: boolean
  } {
    const hasNextPage = results.length > limit

    const items = hasNextPage
      ? results.slice(0, limit)
      : [...results]

    const last = items.at(-1)

    return {
      items,
      nextCursor:
        hasNextPage && last
          ? this.encodeCursor(
            getId(last),
            getTs(last)
          )
          : null,
      hasNextPage,
    }
  }

  toObject() {
    return this.resource
  }
}
