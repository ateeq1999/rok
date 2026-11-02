import type { HttpContext } from '@adonisjs/core/http'
import PointLedger from '#models/point_ledger'
import PointLedgerTransformer from '#transformers/point_ledger_transformer'
import PointsTransformer from '#transformers/helpers/points_transformer'
import { listPointLedgerSchema } from '#validators/rewards'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { success, cursor } from '#helpers/response'

export default class PointsController {
  async balance({ auth, response }: HttpContext) {
    const user = auth.getUserOrFail()
    return success(response, PointsTransformer.transform({ balance: user.points ?? 0, currency: 'JIONI' }))
  }

  async ledger({ auth, request, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const qs = await request.validateUsing(listPointLedgerSchema)
    const limit = qs.limit ?? 20

    const query = PointLedger.query()
      .where('userId', user.id)
      .orderBy('createdAt', 'desc')
      .orderBy('id', 'desc')

    if (qs.cursor) {
      const pos = decodeCursor(qs.cursor)
      if (pos) applyCursorCondition(query, pos, 'createdAt', 'desc')
    }

    const rows = await query.limit(limit + 1)
    const { items, nextCursor } = paginateCursor(
      rows,
      limit,
      (e) => e.createdAt.toISO()!,
      (e) => e.id
    )

    return cursor(response, PointLedgerTransformer.transform(items), nextCursor)
  }
}
