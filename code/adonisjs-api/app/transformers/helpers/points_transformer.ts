import { BaseTransformer } from '@adonisjs/core/transformers'

export interface PointsData {
  balance: number
  currency: string
}

export default class PointsTransformer extends BaseTransformer<PointsData> {
  toObject() {
    return {
      balance: this.resource.balance,
      currency: this.resource.currency,
    }
  }
}
