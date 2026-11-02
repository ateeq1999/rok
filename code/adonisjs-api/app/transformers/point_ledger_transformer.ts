import { BaseTransformer } from '@adonisjs/core/transformers'
import type PointLedger from '#models/point_ledger'

export default class PointLedgerTransformer extends BaseTransformer<PointLedger> {
  toObject() {
    return {
      id: this.resource.id,
      amount: this.resource.amount,
      balanceAfter: this.resource.balanceAfter,
      reason: this.resource.reason,
      refId: this.resource.refId,
      refType: this.resource.refType,
      createdAt: this.resource.createdAt.toISO()!,
    }
  }
}
