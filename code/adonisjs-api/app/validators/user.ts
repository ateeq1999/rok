import vine from '@vinejs/vine'
import { Infer } from '@vinejs/vine/types'

// ---------------------------------------------------------------------------
// Reusable field helpers
// ---------------------------------------------------------------------------
const email = () =>
  vine.string().trim().email().normalizeEmail().maxLength(254)

const password = () =>
  vine.string().minLength(8).maxLength(72)

const tanzaniaPhone = () =>
  vine.string().regex(/^\+?255[0-9]{9}$/)

const fullName = () =>
  vine.string().trim().maxLength(100)

const otpCode = () =>
  vine.string().trim().regex(/^\d{6}$/)

const verificationToken = () =>
  vine.string().trim().minLength(32)

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------
export const registerWithEmailValidator = vine.compile(
  vine.object({
    email: email().unique({ table: 'users', column: 'email' }),
    phone: tanzaniaPhone().unique({ table: 'users', column: 'phone' }).optional(),
    password: password(),
    fullName: fullName().optional(),
    marketingConsent: vine.boolean().optional(),
  })
)
export type RegisterWithEmailInput = Infer<typeof registerWithEmailValidator>

export const registerWithPhoneValidator = vine.compile(
  vine.object({
    phone: tanzaniaPhone(),
    email: email().optional(),
    password: password(),
    fullName: fullName().optional(),
    marketingConsent: vine.boolean().optional(),
  })
)
export type RegisterWithPhoneInput = Infer<typeof registerWithPhoneValidator>

// ---------------------------------------------------------------------------
// Login
// ---------------------------------------------------------------------------
export const loginWithPasswordValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
    password: vine.string(),
  })
)
export type LoginInput = Infer<typeof loginWithPasswordValidator>

// ---------------------------------------------------------------------------
// OTP Login
// ---------------------------------------------------------------------------
export const otpLoginRequestValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
  })
)
export type OtpLoginRequestInput = Infer<typeof otpLoginRequestValidator>

export const otpLoginVerifyValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
    otp: otpCode(),
  })
)
export type OtpLoginVerifyInput = Infer<typeof otpLoginVerifyValidator>

// ---------------------------------------------------------------------------
// Account verification (verify email/phone)
// ---------------------------------------------------------------------------
export const verifyAccountValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
    otp: otpCode().optional(),
    token: verificationToken().optional(),
  })
)
export type VerifyAccountInput = Infer<typeof verifyAccountValidator>

export const resendVerificationValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
  })
)
export type ResendVerificationInput = Infer<typeof resendVerificationValidator>

// ---------------------------------------------------------------------------
// Password reset
// ---------------------------------------------------------------------------
export const passwordResetRequestValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
    method: vine.enum(['otp', 'token']).optional(),
  })
)
export type PasswordResetRequestInput = Infer<typeof passwordResetRequestValidator>

export const passwordResetConfirmValidator = vine.compile(
  vine.object({
    email: email().optional(),
    phone: tanzaniaPhone().optional(),
    otp: otpCode().optional(),
    token: verificationToken().optional(),
    password: password(),
    passwordConfirmation: password().sameAs('password'),
  })
)
export type PasswordResetConfirmInput = Infer<typeof passwordResetConfirmValidator>

// ---------------------------------------------------------------------------
// Profile
// ---------------------------------------------------------------------------
export const updateProfileValidator = vine.compile(
  vine.object({
    fullName: fullName().optional(),
    phone: tanzaniaPhone().optional(),
  })
)
export type UpdateProfileInput = Infer<typeof updateProfileValidator>

// ---------------------------------------------------------------------------
// Change password (authenticated user)
// ---------------------------------------------------------------------------
export const changePasswordValidator = vine.compile(
  vine.object({
    currentPassword: vine.string(),
    password: password(),
    passwordConfirmation: password().sameAs('password'),
  })
)
export type ChangePasswordInput = Infer<typeof changePasswordValidator>

// ---------------------------------------------------------------------------
// Admin user management
// ---------------------------------------------------------------------------
export const adminCreateUserSchema = vine.compile(
  vine.object({
    email: email(),
    phone: tanzaniaPhone().optional(),
    password: password(),
    fullName: fullName().optional(),
    role: vine.enum(['admin', 'member']).optional(),
    status: vine.enum(['active', 'inactive', 'banned']).optional(),
  })
)
export type AdminCreateUserInput = Infer<typeof adminCreateUserSchema>

export const adminUpdateUserSchema = vine.compile(
  vine.object({
    fullName: fullName().optional(),
    phone: tanzaniaPhone().optional(),
    role: vine.enum(['admin', 'member']).optional(),
    status: vine.enum(['active', 'inactive', 'banned']).optional(),
    points: vine.number().optional(),
  })
)
export type AdminUpdateUserInput = Infer<typeof adminUpdateUserSchema>
