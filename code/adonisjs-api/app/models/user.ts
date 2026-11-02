import { hasMany } from '@adonisjs/lucid/orm'
import type { HasMany } from '@adonisjs/lucid/types/relations'
import hash from '@adonisjs/core/services/hash'
import { compose } from '@adonisjs/core/helpers'
import { withAuthFinder } from '@adonisjs/auth/mixins/lucid'
import { AccessToken, DbAccessTokensProvider } from '@adonisjs/auth/access_tokens'
import { WithUlid } from '#mixins/with_ulid'
import { UserSchema } from '#database/schema'
import Ticket from '#models/ticket'
import Order from '#models/order'
import PointLedger from '#models/point_ledger'
import RewardRedemption from '#models/reward_redemption'
import EventRsvp from '#models/event_rsvp'
import ContentLike from '#models/content_like'

const AuthUser = compose(UserSchema, withAuthFinder(() => hash.use('scrypt'), {
  passwordColumnName: 'password',
  uids: ['email', 'phone'],
}))

export default class User extends WithUlid(AuthUser, 'usr') {
  static accessTokens = DbAccessTokensProvider.forModel(User, {
    prefix: 'oat_',
    tokenSecretLength: 40,
    expiresIn: '30 days',
  })
  declare currentAccessToken?: AccessToken

  // Relationships
  @hasMany(() => Ticket)
  declare tickets: HasMany<typeof Ticket>

  @hasMany(() => Order)
  declare orders: HasMany<typeof Order>

  @hasMany(() => PointLedger)
  declare pointLedgers: HasMany<typeof PointLedger>

  @hasMany(() => RewardRedemption)
  declare rewardRedemptions: HasMany<typeof RewardRedemption>

  @hasMany(() => EventRsvp)
  declare rsvps: HasMany<typeof EventRsvp>

  @hasMany(() => ContentLike)
  declare contentLikes: HasMany<typeof ContentLike>

  // Computed
  get initials() {
    const [first, last] = this.fullName ? this.fullName.split(' ') : this.email.split('@')
    if (first && last) return `${first[0]}${last[0]}`.toUpperCase()
    return first?.slice(0, 2).toUpperCase() ?? ''
  }
}
