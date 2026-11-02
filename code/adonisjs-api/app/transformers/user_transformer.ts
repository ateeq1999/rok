import { BaseTransformer } from '@adonisjs/core/transformers'
import type User from '#models/user'

export default class UserTransformer extends BaseTransformer<User> {
  toObject() {
    return {
      ...this.pick(this.resource, ['id', 'fullName', 'email', 'phone', 'emailVerified', 'phoneVerifiedAt', 'phoneVerified'] as const),
      role: this.resource.role ?? null,
      initials: this.resource.initials,
      createdAt: this.resource.createdAt?.toISO() ?? null,
      updatedAt: this.resource.updatedAt?.toISO() ?? null,
    }
  }

  forAuth(token: string) {
    return {
      ...this.pick(this.resource, ['id', 'fullName', 'email', 'phone', 'emailVerified', 'phoneVerifiedAt', 'phoneVerified'] as const),
      role: this.resource.role ?? null,
      initials: this.resource.initials,
      createdAt: this.resource.createdAt?.toISO() ?? null,
      updatedAt: this.resource.updatedAt?.toISO() ?? null,
      token: token,
    }
  }

  async forPublic() {
    return {
      id: this.resource.id,
      fullName: this.resource.fullName,
      initials: this.resource.initials,
    }
  }

  async forAdmin() {
    return {
      ...this.toObject(),
      email: this.resource.email,
      phone: this.resource.phone ?? null,
      points: this.resource.points ?? 0,
      status: (this.resource as any).status ?? 'active',
      emailVerified: this.resource.emailVerified ?? false,
    }
  }
}
