import db from '@adonisjs/lucid/services/db'
import User from '#models/user'
import PointLedger from '#models/point_ledger'

export type PointReason = 'rsvp' | 'purchase' | 'redeem' | 'admin_credit' | 'admin_debit'

class PointsService {
  async earn(
    userId: string,
    amount: number,
    reason: PointReason,
    refId?: string,
    refType?: string
  ) {
    return db.transaction(async (trx) => {
      const user = await User.query()
        .useTransaction(trx)
        .where('id', userId)
        .forUpdate()
        .firstOrFail()

      const newBalance = (user.points ?? 0) + amount
      user.useTransaction(trx)
      user.points = newBalance
      await user.save()

      const entry = new PointLedger()
      entry.useTransaction(trx)
      entry.merge({ userId, amount, balanceAfter: newBalance, reason, refId: refId ?? null, refType: refType ?? null })
      await entry.save()

      return { balance: newBalance, entry }
    })
  }

  async spend(
    userId: string,
    amount: number,
    reason: PointReason,
    refId?: string,
    refType?: string
  ) {
    return db.transaction(async (trx) => {
      const user = await User.query()
        .useTransaction(trx)
        .where('id', userId)
        .forUpdate()
        .firstOrFail()

      const currentBalance = user.points ?? 0
      if (currentBalance < amount) {
        throw new Error('Insufficient points balance')
      }

      const newBalance = currentBalance - amount
      user.useTransaction(trx)
      user.points = newBalance
      await user.save()

      const entry = new PointLedger()
      entry.useTransaction(trx)
      entry.merge({ userId, amount: -amount, balanceAfter: newBalance, reason, refId: refId ?? null, refType: refType ?? null })
      await entry.save()

      return { balance: newBalance, entry }
    })
  }
}

export default new PointsService()
