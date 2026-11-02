import { BaseTransformer } from '@adonisjs/core/transformers'

export interface RsvpData {
  rsvped: boolean
  rsvpCount: number
}

class RsvpTransformer extends BaseTransformer<RsvpData> {
  toObject() {
    return {
      rsvped: this.resource.rsvped,
      rsvpCount: this.resource.rsvpCount,
    }
  }
}

export default RsvpTransformer
