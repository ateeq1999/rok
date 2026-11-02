export const CONTENT_TYPES = ['video', 'article', 'podcast'] as const
export const CONTENT_TIERS = ['free', 'member', 'premium'] as const
export const MEDIA_TYPES = ['thumbnail', 'media'] as const
export const UserRole = {
  ADMIN: 'admin',
  USER: 'user',
} as const
export type UserRole = (typeof UserRole)[keyof typeof UserRole]
