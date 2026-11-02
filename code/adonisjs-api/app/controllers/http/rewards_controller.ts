import type { HttpContext } from '@adonisjs/core/http'
import { DateTime } from 'luxon'
import db from '@adonisjs/lucid/services/db'
import cache from '@adonisjs/cache/services/main'
import Reward from '#models/reward'
import RewardRedemption from '#models/reward_redemption'
import RewardTransformer from '#transformers/reward_transformer'
import PointLedger from '#models/point_ledger'
import { adminCreateRewardSchema, adminUpdateRewardSchema } from '#validators/rewards'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { success, created, paginated, cursor, noContent, unprocessable, badRequest } from '#helpers/response'
import User from '#models/user'

export default class RewardsController {
  /**
 * Get list of active rewards with pagination (offset or cursor) for public.
 * Query parameters:
 * - mode: 'offset' (default) or 'cursor'
 * - page (offset mode): page number, default 1
 * - limit (both modes): items per page, default 20
 * - cursor (cursor mode): cursor string for next page
 */
  async active({ request, response }: HttpContext) {
    const { mode = 'offset', page = 1, limit = 20, cursor: cursorParam } = request.qs()

    const baseQuery = Reward.query()
      .where('status', 'active')
      .where((q) => q.whereNull('expiresAt').orWhere('expiresAt', '>', DateTime.now().toSQL()!))
      .orderBy('pointsCost', 'asc')
      .orderBy('id', 'asc') // ensure deterministic ordering for cursor

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      const rewards = await baseQuery.paginate(page, limit)
      return paginated(response, RewardTransformer.transform(rewards), rewards.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      cursorQuery.orderBy('pointsCost', 'asc').orderBy('id', 'asc')

      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) applyCursorCondition(cursorQuery, pos, 'pointsCost', 'asc')
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (reward) => reward.pointsCost.toString(),
        (reward) => reward.id
      )

      return cursor(response, RewardTransformer.transform(items), nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  /**
   * List all rewards with pagination (offset or cursor) - admin.
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20
   * - cursor (cursor mode): cursor string for next page
   */
  async index({ request, response }: HttpContext) {
    const { mode = 'offset', page = 1, limit = 20, cursor: cursorParam } = request.qs()

    const baseQuery = Reward.query().orderBy('createdAt', 'desc')
    const cacheKey = `rewards:pub:${mode}:p${page}:l${limit}:${cursorParam ?? ''}`

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      const cached = await cache.getOrSet({
        key: cacheKey,
        tags: ['rewards'],
        factory: async () => {
          const rewards = await baseQuery.paginate(page, limit)
          return { data: RewardTransformer.transform(rewards), meta: rewards.getMeta() }
        },
      })
      return paginated(response, cached.data, cached.meta)
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      cursorQuery.orderBy('createdAt', 'desc').orderBy('id', 'desc')

      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) applyCursorCondition(cursorQuery, pos, 'createdAt', 'desc')
      }

      const cached = await cache.getOrSet({
        key: cacheKey,
        tags: ['rewards'],
        factory: async () => {
          const rows = await cursorQuery.limit(limit + 1)
          const { items, nextCursor: nc } = paginateCursor(
            rows, limit,
            (reward) => reward.createdAt.toString(),
            (reward) => reward.id
          )
          return { data: RewardTransformer.transform(items), nextCursor: nc }
        },
      })
      return cursor(response, cached.data, cached.nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  async store({ request, response }: HttpContext) {
    const data = await request.validateUsing(adminCreateRewardSchema)
    const reward = await Reward.create({
      name: data.name,
      description: data.description ?? null,
      pointsCost: data.pointsCost,
      quantityTotal: data.quantityTotal ?? null,
      expiresAt: data.expiresAt ?? null,
      status: data.status ?? 'active',
      quantityRedeemed: 0,
    })
    await cache.deleteByTag({ tags: ['rewards'] })
    return created(response, RewardTransformer.transform(reward))
  }

  async update({ params, request, response }: HttpContext) {
    const reward = await Reward.findOrFail(params.id)
    const data = await request.validateUsing(adminUpdateRewardSchema)
    const patch: Record<string, unknown> = {}
    if (data.name !== undefined) patch.name = data.name
    if (data.description !== undefined) patch.description = data.description
    if (data.pointsCost !== undefined) patch.pointsCost = data.pointsCost
    if (data.quantityTotal !== undefined) patch.quantityTotal = data.quantityTotal
    if (data.status !== undefined) patch.status = data.status
    if (data.expiresAt !== undefined) patch.expiresAt = data.expiresAt
    await reward.merge(patch).save()
    await cache.deleteByTag({ tags: ['rewards'] })
    return success(response, RewardTransformer.transform(reward))
  }

  async redeem({ params, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()
    const reward = await Reward.findOrFail(params.id)

    if (!reward.isAvailable) {
      return unprocessable(response, 'Reward is not available for redemption', undefined, 'reward_unavailable')
    }

    let insufficientPoints = false

    await db.transaction(async (trx) => {
      // Re-read with FOR UPDATE to prevent double-spend race condition
      const lockedUser = await User.query()
        .useTransaction(trx)
        .where('id', user.id)
        .forUpdate()
        .firstOrFail()

      const balance = lockedUser.points ?? 0
      if (balance < reward.pointsCost) {
        insufficientPoints = true
        return
      }

      lockedUser.points = balance - reward.pointsCost
      await lockedUser.save()
      user.points = lockedUser.points

      const entry = new PointLedger()
      entry.useTransaction(trx)
      entry.merge({
        userId: user.id,
        amount: -reward.pointsCost,
        balanceAfter: lockedUser.points,
        reason: 'redeem',
        refId: reward.id,
        refType: 'reward',
      })
      await entry.save()

      reward.useTransaction(trx)
      reward.quantityRedeemed += 1
      await reward.save()

      const redemption = new RewardRedemption()
      redemption.useTransaction(trx)
      redemption.merge({
        userId: user.id,
        rewardId: reward.id,
        pointsSpent: reward.pointsCost,
        redeemedAt: DateTime.now(),
      })
      await redemption.save()
    })

    if (insufficientPoints) {
      const balance = user.points ?? 0
      return unprocessable(
        response,
        `Insufficient points. You have ${balance} but need ${reward.pointsCost}`,
        undefined,
        'insufficient_points'
      )
    }

    return success(
      response,
      {
        reward: new RewardTransformer(reward).toObject(),
        pointsBalance: user.points,
        redeemed: true,
      },
      { message: 'Reward redeemed successfully' }
    )
  }

  async destroy({ params, response }: HttpContext) {
    const reward = await Reward.findOrFail(params.id)
    await reward.delete()
    await cache.deleteByTag({ tags: ['rewards'] })
    return noContent(response)
  }
}
