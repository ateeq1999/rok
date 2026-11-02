import { BaseTransformer } from '@adonisjs/core/transformers'

export interface AttendanceData {
  events: unknown[]
  total: number
}

export default class AttendanceTransformer extends BaseTransformer<AttendanceData> {
  toObject() {
    return {
      events: this.resource.events,
      total: this.resource.total,
    }
  }
}
