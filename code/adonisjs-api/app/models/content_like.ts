import { belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { ContentLikeSchema } from '#database/schema'
import User from '#models/user'
import Content from '#models/content'

export default class ContentLike extends WithUlid(ContentLikeSchema, 'clk') {
  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @belongsTo(() => Content)
  declare content: BelongsTo<typeof Content>
}
