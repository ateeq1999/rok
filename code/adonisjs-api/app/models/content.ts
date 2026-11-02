import { hasMany } from '@adonisjs/lucid/orm'
import type { HasMany } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { ContentSchema } from '#database/schema'
import ContentLike from '#models/content_like'

export default class Content extends WithUlid(ContentSchema, 'cnt') {
  @hasMany(() => ContentLike)
  declare likes: HasMany<typeof ContentLike>
}
