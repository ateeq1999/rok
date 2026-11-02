import { middleware } from '#start/kernel'
import { authThrottle, apiThrottle } from '#start/limiter'
import router from '@adonisjs/core/services/router'
import { controllers } from '#generated/controllers'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import yaml from 'js-yaml'

// ─────────────────────────────────────────────
// Health
// ─────────────────────────────────────────────

router.get('/health/live', [controllers.HealthChecks, 'live']).as('health.live')
router.get('/health/ready', [controllers.HealthChecks, 'ready']).as('health.ready')
router.get('/health', [controllers.HealthChecks, 'live']).as('health.ping')

// ─────────────────────────────────────────────
// OpenAPI spec
// ─────────────────────────────────────────────

const openapiPath = resolve(import.meta.dirname, '../../../openapi.yaml')
let openapiJson: object | null = null

try {
  openapiJson = yaml.load(readFileSync(openapiPath, 'utf-8')) as object
} catch {
  // spec not available in this environment
}

router.get('/docs.json', ({ response }) => {
  if (!openapiJson) return response.notFound({ ok: false, error: { message: 'OpenAPI spec not available' } })
  return response.header('Content-Type', 'application/json').send(JSON.stringify(openapiJson))
})

router.get('/docs.yaml', ({ response }) => {
  try {
    const raw = readFileSync(openapiPath, 'utf-8')
    return response.header('Content-Type', 'text/yaml').send(raw)
  } catch {
    return response.notFound({ ok: false, error: { message: 'OpenAPI spec not available' } })
  }
})

// ─────────────────────────────────────────────
// Auth  —  /api/v1/auth
// ─────────────────────────────────────────────

router.group(() => {
  // Registration
  router.post('/register/email', [controllers.auth.Auth, 'registerWithEmail']).use(authThrottle)
  router.post('/register/phone', [controllers.auth.Auth, 'registerWithPhone']).use(authThrottle)

  // Password login
  router.post('/login', [controllers.auth.Auth, 'login']).use(authThrottle)
  router.post('/refresh', [controllers.auth.Auth, 'refresh']).use(middleware.auth())
  router.post('/logout', [controllers.auth.Auth, 'logout']).use(middleware.auth())
  router.get('/me', [controllers.auth.Auth, 'me']).use(middleware.auth())

  // OTP login
  router.post('/otp/request', [controllers.auth.Otp, 'request']).use(authThrottle)
  router.post('/otp/verify', [controllers.auth.Otp, 'verify']).use(authThrottle)

  // Email / phone verification
  router.post('/verify', [controllers.auth.Verification, 'verify'])
  router.post('/verify/resend', [controllers.auth.Verification, 'resend'])
  router.get('/email/verify/:token', [controllers.auth.EmailVerification, 'verify'])
  router.post('/email/verify/resend', [controllers.auth.EmailVerification, 'resend']).use(middleware.auth())

  // Password reset
  router.post('/password/reset/request', [controllers.auth.PasswordResets, 'request']).use(authThrottle)
  router.post('/password/reset/confirm', [controllers.auth.PasswordResets, 'confirm']).use(authThrottle)
}).prefix('/api/v1/auth')

// ─────────────────────────────────────────────
// Events
// ─────────────────────────────────────────────

router.get('/api/v1/events', [controllers.http.Events, 'index']).use(apiThrottle)
router.post('/api/v1/events', [controllers.http.Events, 'store']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/events/:id', [controllers.http.Events, 'show']).use(apiThrottle)
router.patch('/api/v1/events/:id', [controllers.http.Events, 'update']).use([middleware.auth(), middleware.admin()])
router.delete('/api/v1/events/:id', [controllers.http.Events, 'destroy']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/events/:id/tickets', [controllers.http.TicketTypes, 'index']).use(apiThrottle)
router.get('/api/v1/events/:id/lineup', () => ({ ok: true, data: [], meta: { timestamp: new Date().toISOString() } }))
router.get('/api/v1/events/:id/attendees', [controllers.http.Attendees, 'index']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/events/:id/attendees.csv', [controllers.http.Attendees, 'csv']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/events/:id/rsvp', [controllers.http.Rsvp, 'rsvp']).use(middleware.auth())
router.delete('/api/v1/events/:id/rsvp', [controllers.http.Rsvp, 'cancel']).use(middleware.auth())

// ─────────────────────────────────────────────
// Orders
// ─────────────────────────────────────────────

router.post('/api/v1/orders', [controllers.http.Orders, 'store']).use(middleware.auth())
router.post('/api/v1/orders/:id/issue', [controllers.http.Orders, 'issueTickets']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/orders/:id', [controllers.http.Orders, 'show']).use(middleware.auth())
router.post('/api/v1/orders/:id/cancel', [controllers.http.Orders, 'cancel']).use(middleware.auth())

// ─────────────────────────────────────────────
// Tickets
// ─────────────────────────────────────────────

router.get('/api/v1/tickets/mine', [controllers.http.Tickets, 'mine']).use(middleware.auth())
router.get('/api/v1/tickets/:id', [controllers.http.Tickets, 'show']).use(middleware.auth())
router.post('/api/v1/tickets/scan', [controllers.http.Tickets, 'scan']).use([middleware.auth(), middleware.admin()])

// ─────────────────────────────────────────────
// Content
// ─────────────────────────────────────────────

router.get('/api/v1/content', [controllers.http.Content, 'index']).use(apiThrottle)
router.post('/api/v1/content', [controllers.http.Content, 'store']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/content/upload', [controllers.http.Content, 'upload']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/content/:id', [controllers.http.Content, 'show']).use(apiThrottle)
router.patch('/api/v1/content/:id', [controllers.http.Content, 'update']).use([middleware.auth(), middleware.admin()])
router.delete('/api/v1/content/:id', [controllers.http.Content, 'destroy']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/content/:id/like', [controllers.http.Content, 'like']).use(middleware.auth())
router.post('/api/v1/content/:id/publish', [controllers.http.Content, 'publish']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/content/:id/unpublish', [controllers.http.Content, 'unpublish']).use([middleware.auth(), middleware.admin()])

// ─────────────────────────────────────────────
// Users
// ─────────────────────────────────────────────

router.get('/api/v1/users/me', [controllers.http.Profile, 'show']).use(middleware.auth())
router.patch('/api/v1/users/me', [controllers.http.Profile, 'update']).use(middleware.auth())
router.post('/api/v1/users/me/password', [controllers.http.Profile, 'changePassword']).use(middleware.auth())
router.get('/api/v1/users/me/points', [controllers.http.Profile, 'points']).use(middleware.auth())
router.get('/api/v1/users/me/attendance', [controllers.http.Profile, 'attendance']).use(middleware.auth())
router.get('/api/v1/users', [controllers.http.Users, 'index']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/users', [controllers.http.Users, 'store']).use([middleware.auth(), middleware.admin()])
router.get('/api/v1/users/:id', [controllers.http.Users, 'show']).use([middleware.auth(), middleware.admin()])
router.patch('/api/v1/users/:id', [controllers.http.Users, 'update']).use([middleware.auth(), middleware.admin()])

// ─────────────────────────────────────────────
// Points
// ─────────────────────────────────────────────

router.get('/api/v1/points/balance', [controllers.http.Points, 'balance']).use(middleware.auth())
router.get('/api/v1/points/ledger', [controllers.http.Points, 'ledger']).use(middleware.auth())

// ─────────────────────────────────────────────
// Rewards
// ─────────────────────────────────────────────

router.get('/api/v1/rewards', [controllers.http.Rewards, 'index']).use(apiThrottle)
router.post('/api/v1/rewards', [controllers.http.Rewards, 'store']).use([middleware.auth(), middleware.admin()])
router.patch('/api/v1/rewards/:id', [controllers.http.Rewards, 'update']).use([middleware.auth(), middleware.admin()])
router.delete('/api/v1/rewards/:id', [controllers.http.Rewards, 'destroy']).use([middleware.auth(), middleware.admin()])
router.post('/api/v1/rewards/:id/redeem', [controllers.http.Rewards, 'redeem']).use(middleware.auth())
