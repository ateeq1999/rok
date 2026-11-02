// app/helpers/pagination.ts

export interface CursorPosition {
  ts: string
  id: string
}

export function encodeCursor(id: string | number, ts: string): string {
  return Buffer.from(JSON.stringify({ ts, id: String(id) })).toString('base64url')
}

export function decodeCursor(cursor: string): CursorPosition | null {
  try {
    const decoded = JSON.parse(Buffer.from(cursor, 'base64url').toString('utf8'))
    if (decoded && typeof decoded.ts === 'string' && decoded.id !== undefined) {
      return { ts: decoded.ts, id: String(decoded.id) }
    }
    return null
  } catch {
    return null
  }
}

/**
 * Apply cursor condition to a query for ordering by a timestamp field and ID.
 * Assumes the field is non-null for rows before the cursor.
 */
export function applyCursorCondition(
  query: { where: Function },
  pos: CursorPosition,
  field: string,
  direction: 'asc' | 'desc'
): void {
  const op = direction === 'asc' ? '>' : '<'
  query.where((q: any) => {
    q.where(field, op, pos.ts).orWhere((q2: any) => {
      q2.where(field, pos.ts).andWhere('id', op, pos.id)
    })
  })
}

/**
 * Paginate using cursor.
 * @param rows - Rows fetched (limit+1)
 * @param limit - Limit per page
 * @param tsGetter - Function to extract timestamp string (may return null)
 * @param idGetter - Function to extract ID
 * @returns Items and nextCursor (null if no more pages)
 */
export function paginateCursor<T>(
  rows: T[],
  limit: number,
  tsGetter: (item: T) => string | null,
  idGetter: (item: T) => string | number
): { items: T[]; nextCursor: string | null } {
  const hasMore = rows.length > limit
  const items = hasMore ? rows.slice(0, limit) : rows
  const last = items.at(-1)
  let nextCursor: string | null = null
  if (hasMore && last) {
    const ts = tsGetter(last)
    if (ts) {
      nextCursor = encodeCursor(idGetter(last), ts)
    }
  }
  return { items, nextCursor }
}
