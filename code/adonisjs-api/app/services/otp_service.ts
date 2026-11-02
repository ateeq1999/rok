import { createHash, randomBytes, randomInt } from 'node:crypto'
import { DateTime } from 'luxon'
import OtpCode, { type OtpType } from '#models/otp_code'

const OTP_EXPIRY_MAP: Record<OtpType, { minutes?: number; hours?: number }> = {
  email_verify: { hours: 24 },
  phone_verify: { minutes: 10 },
  password_reset_otp: { minutes: 15 },
  password_reset_token: { hours: 24 },
  otp_login: { minutes: 10 },
}

export class OtpService {
  async generateOtp(): Promise<string> {
    return String(randomInt(100_000, 999_999))
  }

  async generateToken(): Promise<string> {
    return randomBytes(32).toString('hex')
  }

  private hash(value: string): string {
    return createHash('sha256').update(value).digest('hex')
  }

  private getExpiry(type: OtpType) {
    return DateTime.now().plus(OTP_EXPIRY_MAP[type])
  }

  private async invalidateExisting(userId: string, type: OtpType): Promise<void> {
    await OtpCode.query()
      .where('userId', userId)
      .where('type', type)
      .whereNull('usedAt')
      .update({ used_at: DateTime.now().toSQL() })
  }

  async create(userId: string, type: OtpType, value: string): Promise<void> {
    await this.invalidateExisting(userId, type)
    await OtpCode.create({
      userId,
      type,
      code: this.hash(value),
      expiresAt: this.getExpiry(type),
    })
  }

  async verify(type: OtpType, value: string, userId?: string): Promise<OtpCode | null> {
    const query = OtpCode.query()
      .where('type', type)
      .where('code', this.hash(value))
      .whereNull('usedAt')
      .where('expiresAt', '>', DateTime.now().toSQL()!)

    if (userId) {
      query.where('userId', userId)
    }

    const record = await query.first()
    if (record) {
      record.merge({ usedAt: DateTime.now() })
      await record.save()
    }

    return record
  }
}
