import type { HttpContext } from '@adonisjs/core/http'
import cache from '@adonisjs/cache/services/main'
import Content from '#models/content'
import ContentLike from '#models/content_like'
import ContentTransformer from '#transformers/content_transformer'
import { ErrorCode } from '#enums/error_codes'
import {
  createContentSchema,
  updateContentSchema,
  uploadContentMediaSchema,
} from '#validators/content'
import { decodeCursor, applyCursorCondition, paginateCursor } from '#helpers/pagination'
import { DateTime } from 'luxon'
import { randomBytes } from 'node:crypto'
import drive from '@adonisjs/drive/services/main'
import { paginated, cursor, success, created, noContent, notFound, conflict, unprocessable, badRequest } from '#helpers/response'

export default class ContentController {
  /**
   * List contents with pagination (offset or cursor).
   * For admin: may include unpublished contents.
   * For public: only published contents.
   * Query parameters:
   * - mode: 'offset' (default) or 'cursor'
   * - page (offset mode): page number, default 1
   * - limit (both modes): items per page, default 20
   * - cursor (cursor mode): cursor string for next page
   * - type, tier: filters
   */
  async index({ request, response }: HttpContext) {
    const { mode = 'offset', page = 1, limit = 20, cursor: cursorParam, type, tier } = request.qs()

    const baseQuery = Content.query().orderBy('createdAt', 'desc')
    if (type) baseQuery.where('type', type)
    if (tier) baseQuery.where('tier', tier)

    // For public endpoints, only published content
    const isAdminRoute = request.url().includes('/admin')
    if (!isAdminRoute) {
      baseQuery.whereNotNull('publishedAt')
    }

    const cacheKey = `content:${isAdminRoute ? 'admin' : 'pub'}:${mode}:p${page}:l${limit}:${cursorParam ?? ''}:${type ?? ''}:${tier ?? ''}`

    // ─── Offset Pagination ─────────────────────────────────────────
    if (mode === 'offset') {
      if (!isAdminRoute) {
        const cached = await cache.getOrSet({
          key: cacheKey,
          tags: ['content'],
          factory: async () => {
            const contents = await baseQuery.paginate(page, limit)
            return { data: ContentTransformer.transform(contents), meta: contents.getMeta() }
          },
        })
        return paginated(response, cached.data, cached.meta)
      }
      const contents = await baseQuery.paginate(page, limit)
      return paginated(response, ContentTransformer.transform(contents), contents.getMeta())
    }

    // ─── Cursor Pagination ─────────────────────────────────────────
    if (mode === 'cursor') {
      const cursorQuery = baseQuery.clone()
      if (!isAdminRoute) {
        cursorQuery.orderBy('publishedAt', 'desc').orderBy('id', 'desc')
      } else {
        cursorQuery.orderBy('createdAt', 'desc').orderBy('id', 'desc')
      }

      if (cursorParam) {
        const pos = decodeCursor(cursorParam)
        if (pos) {
          const orderColumn = isAdminRoute ? 'createdAt' : 'publishedAt'
          applyCursorCondition(cursorQuery, pos, orderColumn, 'desc')
        }
      }

      if (!isAdminRoute) {
        const cached = await cache.getOrSet({
          key: cacheKey,
          tags: ['content'],
          factory: async () => {
            const rows = await cursorQuery.limit(limit + 1)
            const { items, nextCursor: nc } = paginateCursor(
              rows, limit,
              (content) => content.publishedAt!.toString(),
              (content) => content.id
            )
            return { data: items.map((item) => new ContentTransformer(item).toObject()), nextCursor: nc }
          },
        })
        return cursor(response, cached.data, cached.nextCursor)
      }

      const rows = await cursorQuery.limit(limit + 1)
      const { items, nextCursor } = paginateCursor(
        rows,
        limit,
        (content) => (isAdminRoute ? content.createdAt.toString() : content.publishedAt!.toString()),
        (content) => content.id
      )

      const transformed = items.map((item) => new ContentTransformer(item).toObject())
      return cursor(response, transformed, nextCursor)
    }

    return badRequest(response, 'Invalid pagination mode. Use "offset" or "cursor"')
  }

  /**
   * Like a content
   */
  async like({ params, auth, response }: HttpContext) {
    const user = auth.getUserOrFail()

    const content = await Content.query()
      .where('id', params.id)
      .whereNotNull('publishedAt')
      .first()

    if (!content) {
      return notFound(response, 'Content not found or not published', ErrorCode.NOT_FOUND)
    }

    const existing = await ContentLike.query()
      .where('contentId', content.id)
      .where('userId', user.id)
      .first()

    if (existing) {
      return conflict(response, 'Content already liked', undefined, ErrorCode.ALREADY_LIKED)
    }

    await ContentLike.create({ contentId: content.id, userId: user.id })
    content.likeCount += 1
    await content.save()

    return success(response, { liked: true, likeCount: content.likeCount + 1 })
  }

  /**
   * Show a single content
   */
  async show({ params, response }: HttpContext) {
    const content = await Content.findOrFail(params.id)
    return success(response, ContentTransformer.transform(content))
  }

  /**
   * Create a new content (admin)
   */
  async store({ request, response }: HttpContext) {
    const data = await request.validateUsing(createContentSchema)
    const content = await Content.create(data)
    await cache.deleteByTag({ tags: ['content'] })
    return created(response, ContentTransformer.transform(content))
  }

  /**
   * Update content (admin)
   */
  async update({ params, request, response }: HttpContext) {
    const content = await Content.findOrFail(params.id)
    const data = await request.validateUsing(updateContentSchema)
    await content.merge(data).save()
    await cache.deleteByTag({ tags: ['content'] })
    return success(response, ContentTransformer.transform(content))
  }

  /**
   * Publish content
   */
  async publish({ params, response }: HttpContext) {
    const content = await Content.findOrFail(params.id)

    if (content.publishedAt) {
      return unprocessable(response, 'Content is already published', undefined, ErrorCode.UNPROCESSABLE_ENTITY)
    }

    content.publishedAt = DateTime.now()
    await content.save()
    await cache.deleteByTag({ tags: ['content'] })

    return success(response, ContentTransformer.transform(content))
  }

  /**
   * Unpublish content
   */
  async unpublish({ params, response }: HttpContext) {
    const content = await Content.findOrFail(params.id)
    content.publishedAt = null
    await content.save()
    await cache.deleteByTag({ tags: ['content'] })
    return success(response, ContentTransformer.transform(content))
  }

  /**
   * Delete content
   */
  async destroy({ params, response }: HttpContext) {
    const content = await Content.findOrFail(params.id)
    await content.delete()
    await cache.deleteByTag({ tags: ['content'] })
    return noContent(response)
  }

  /**
   * Upload media (image or video)
   */
  async upload({ request, response }: HttpContext) {
    const data = await request.validateUsing(uploadContentMediaSchema)

    const token = randomBytes(8).toString('hex')
    const fileName = `${data.uploadType ? 'media' : 'thumbnails'}/${token}.${data.file.extname}`

    await data.file.moveToDisk(fileName)
    const url = await drive.use().getUrl(fileName)

    return success(response, { url })
  }
}
