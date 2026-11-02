import { NormalizeConstructor } from '@adonisjs/core/types/helpers'
import { BaseModel, beforeCreate } from '@adonisjs/lucid/orm'
import { ulid } from "@std/ulid";

export function WithUlid<T extends NormalizeConstructor<typeof BaseModel>>(
  superclass: T,
  prefix: string
) {
  class WithUlidMixin extends superclass {
    static selfAssignPrimaryKey = true
    @beforeCreate()
    static async assignId(instance: InstanceType<T>) {
      if (!(instance as any).id) {
        ; (instance as any).id = `${prefix}_${ulid()}`
      }
    }
  }

  return WithUlidMixin
}
