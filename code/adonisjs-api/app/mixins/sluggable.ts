import { NormalizeConstructor } from '@adonisjs/core/types/helpers'
import { BaseModel, beforeCreate, beforeUpdate, column } from '@adonisjs/lucid/orm'
import { slugify, SlugifyOptions } from '#utils/slugify'

export interface SluggableOptions extends SlugifyOptions {
  source: string
  column?: string
}

export function Sluggable<T extends NormalizeConstructor<typeof BaseModel>>(
  superclass: T,
  opts: SluggableOptions
) {
  const slugColumn = opts.column ?? 'slug'

  class SluggableMixin extends superclass {
    @column()
    declare slug: string

    static async #findUniqueSlug(base: string, excludeId?: string): Promise<string> {
      const slug = slugify(base, {
        separator: opts.separator,
        maxLength: opts.maxLength,
        lower: opts.lower,
      })

      for (let suffix = 0; suffix < 100; suffix++) {
        const candidate = suffix === 0 ? slug : `${slug}-${suffix}`

        const exists = await (this as unknown as typeof BaseModel)
          .query()
          .where(slugColumn, candidate)
          .if(!!excludeId, (q) => q.whereNot('id', excludeId!))
          .first()

        if (!exists) return candidate
      }

      return `${slug}-${Math.random().toString(36).slice(2, 7)}`
    }

    @beforeCreate()
    static async generateSlug(instance: InstanceType<T> & SluggableMixin) {
      if ((instance as any)[slugColumn]) return

      const source = (instance as any)[opts.source]
      if (!source) return

        ; (instance as any)[slugColumn] = await SluggableMixin.#findUniqueSlug(source)
    }

    @beforeUpdate()
    static async regenerateSlug(instance: InstanceType<T> & SluggableMixin) {
      if (!instance.$dirty[opts.source]) return
      if ((instance.$dirty as any)[slugColumn]) return

      const source = (instance as any)[opts.source]
      if (!source) return

      const id = String(instance.$getAttribute('id'))
        ; (instance as any)[slugColumn] = await SluggableMixin.#findUniqueSlug(source, id)
    }
  }

  return SluggableMixin
}
