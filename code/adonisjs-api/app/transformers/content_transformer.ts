import { BaseTransformer } from '@adonisjs/core/transformers'
import type Content from '#models/content'

export type ContentType = 'video' | 'article' | 'podcast'
export type ContentTier = 'free' | 'member' | 'premium'

export default class ContentTransformer extends BaseTransformer<Content> {
  toObject() {
    return {
      ...this.pick(this.resource, [
        'id',
        'title',
        'slug',
        'type',
        'tier',
        'description',
        'thumbnailUrl',
        'durationSeconds',
        'likeCount',
      ]),
      publishedAt: this.resource.publishedAt?.toISO() ?? null,
      createdAt: this.resource.createdAt.toISO()!,
    }
  }

  forDetail(likedByUser = false) {
    return {
      ...this.toObject(),
      body: this.resource.body,
      videoUrl: this.resource.videoUrl,
      likedByUser,
    }
  }
}
