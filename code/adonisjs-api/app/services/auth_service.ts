import { inject } from '@adonisjs/core'
import User from '#models/user'
import { VerificationService } from '#services/verification_service'
import type { LoginInput } from '#validators/user'
import { DateTime } from 'luxon'

interface RegisterData {
  email?: string
  phone?: string | null
  password: string
  fullName?: string | null
  marketingConsent?: boolean | null
}

@inject()
export class AuthService {
  constructor(protected verificationService: VerificationService) { }

  async createTokenForUser(user: User): Promise<string> {
    const token = await User.accessTokens.create(user, ['*'], { name: 'auth-token' })
    return token.value!.release()
  }

  async register(data: RegisterData): Promise<{ token: string; user: User }> {
    const user = await User.create({
      email: data.email,
      phone: data.phone ?? null,
      password: data.password,
      fullName: data.fullName ?? null,
    })

    await this.verificationService.sendChallenge(
      {
        userId: user.id,
        email: data.email,
        phone: data.phone,
        name: data.fullName,
      },
      'verify',
      'token'
    )

    const tokenValue = await this.createTokenForUser(user)
    return { token: tokenValue, user }
  }

  async login(input: LoginInput): Promise<{ token: string; user: User }> {
    const where = input.email ?? input.phone
    if (!where) throw new Error('Email or phone is required')

    const user = await User.verifyCredentials(where, input.password)
    const tokenValue = await this.createTokenForUser(user)
    return { token: tokenValue, user }
  }

  async verifyAccount(
    user: User,
    type: 'email_verify' | 'phone_verify',
    value: string
  ): Promise<boolean> {
    const record = await this.verificationService.verifyChallenge(type, value, user.id)

    if (!record) return false

    if (type === 'email_verify') {
      user.emailVerifiedAt = DateTime.now()
      user.emailVerified = true
    } else {
      user.phoneVerifiedAt = DateTime.now()
      user.phoneVerified = true
    }
    await user.save()

    return true
  }

  async refreshToken(user: User): Promise<{ token: string }> {
    const oldToken = user.currentAccessToken
    const newTokenValue = await this.createTokenForUser(user)

    if (oldToken) {
      await User.accessTokens.delete(user, oldToken.identifier)
    }

    return { token: newTokenValue }
  }

  async logout(user: User): Promise<void> {
    if (user.currentAccessToken) {
      await User.accessTokens.delete(user, user.currentAccessToken.identifier)
    }
  }
}
