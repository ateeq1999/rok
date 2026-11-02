import { belongsTo } from '@adonisjs/lucid/orm'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'
import { WithUlid } from '#mixins/with_ulid'
import { OtpCodeSchema } from '#database/schema'
import User from '#models/user'

export type OtpType =
  | 'email_verify'       // email verification during registration
  | 'phone_verify'       // phone verification during registration
  | 'password_reset_otp' // password reset using OTP (numeric)
  | 'password_reset_token' // password reset using token (link)
  | 'otp_login'           // login using OTP (email/phone)

export default class OtpCode extends WithUlid(OtpCodeSchema, 'otp') {
  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>
}
