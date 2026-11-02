import type { HttpContext } from '@adonisjs/core/http'
import User from '#models/user'
import UserTransformer from '#transformers/user_transformer'
import { adminCreateUserSchema, adminUpdateUserSchema } from '#validators/user'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { success, created, paginated, cursor, badRequest } from '#helpers/response'

export default class UsersController {
  /**
   * List users with pagination (offset or cursor).
   * Admin only.
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20
   * - cursor (cursor mode): cursor string for next page
   * - search, role, status: filters
   */
  async index({ request, response }: HttpContext) {
    const {
      mode = 'offset',
      page = 1,
      limit = 20,
      cursor: cursorParam,
      search,
      role,
      status,
    } = request.qs()

    // Base query with filters
    const baseQuery = User.query().orderBy('createdAt', 'desc')
    if (search) {
      const term = `%${search}%`
      baseQuery.where((q) => {
        q.whereLike('email', term).orWhereLike('full_name', term)
      })
    }
    if (role) baseQuery.where('role', role)
    if (status) baseQuery.where('status', status)

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      const users = await baseQuery.paginate(page, limit)
      return paginated(response, UserTransformer.transform(users), users.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      cursorQuery.orderBy('createdAt', 'desc').orderBy('id', 'desc')

      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) applyCursorCondition(cursorQuery, pos, 'createdAt', 'desc')
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (user) => user.createdAt.toString(),
        (user) => user.id
      )

      return cursor(response, UserTransformer.transform(items), nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  async show({ params, response }: HttpContext) {
    const user = await User.findOrFail(params.id)
    return success(response, await new UserTransformer(user).forAdmin())
  }

  async store({ request, response }: HttpContext) {
    const data = await request.validateUsing(adminCreateUserSchema)

    const user = await User.create({
      email: data.email,
      password: data.password,
      fullName: data.fullName,
      phone: data.phone,
      role: data.role ?? 'member',
      status: data.status ?? 'active',
      emailVerified: false,
      marketingConsent: false,
    })

    return created(response, await new UserTransformer(user).forAdmin())
  }

  async update({ params, request, response }: HttpContext) {
    const user = await User.findOrFail(params.id)
    const data = await request.validateUsing(adminUpdateUserSchema)

    user.merge({
      ...(data.fullName !== undefined && { fullName: data.fullName }),
      ...(data.phone !== undefined && { phone: data.phone }),
      ...(data.role !== undefined && { role: data.role }),
      ...(data.status !== undefined && { status: data.status }),
      ...(data.points !== undefined && { points: data.points }),
    })

    await user.save()
    return success(response, await new UserTransformer(user).forAdmin())
  }
}
