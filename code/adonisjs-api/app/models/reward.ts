import { DateTime } from 'luxon'
import { hasMany } from '@adonisjs/lucid/orm'
import type { HasMany } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { RewardSchema } from '#database/schema'
import RewardRedemption from '#models/reward_redemption'

export default class Reward extends WithUlid(RewardSchema, 'rew') {
  @hasMany(() => RewardRedemption)
  declare redemptions: HasMany<typeof RewardRedemption>

  get isAvailable(): boolean {
    if (this.status !== 'active') return false
    if (this.expiresAt && this.expiresAt < DateTime.now()) return false
    if (this.quantityTotal !== null && this.quantityRedeemed >= this.quantityTotal) return false
    return true
  }
}
