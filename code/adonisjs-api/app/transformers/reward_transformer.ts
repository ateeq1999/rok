import { BaseTransformer } from '@adonisjs/core/transformers'
import type Reward from '#models/reward'

export default class RewardTransformer extends BaseTransformer<Reward> {
  toObject() {
    return {
      id: this.resource.id,
      name: this.resource.name,
      description: this.resource.description,
      pointsCost: this.resource.pointsCost,
      quantityTotal: this.resource.quantityTotal,
      quantityRedeemed: this.resource.quantityRedeemed,
      quantityAvailable:
        this.resource.quantityTotal !== null
          ? Math.max(0, this.resource.quantityTotal - this.resource.quantityRedeemed)
          : null,
      status: this.resource.status,
      expiresAt: this.resource.expiresAt?.toISO() ?? null,
      createdAt: this.resource.createdAt.toISO()!,
    }
  }
}
