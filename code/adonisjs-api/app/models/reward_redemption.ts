import { belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { RewardRedemptionSchema } from '#database/schema'
import User from '#models/user'
import Reward from '#models/reward'

export default class RewardRedemption extends WithUlid(RewardRedemptionSchema, 'rdm') {
  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @belongsTo(() => Reward)
  declare reward: BelongsTo<typeof Reward>
}
