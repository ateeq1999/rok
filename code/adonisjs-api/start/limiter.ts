import limiter from '@adonisjs/limiter/services/main'

/**
 * Auth endpoints: 10 requests / minute per IP.
 * Protects register and login from brute-force and credential stuffing.
 * Failed attempts beyond the limit are blocked for an additional 10 minutes.
 */
export const authThrottle = limiter.define('auth', (ctx) => {
  return limiter
    .allowRequests(10)
    .every('1 minute')
    .usingKey(`auth_${ctx.request.ip()}`)
    .blockFor('10 mins')
    .limitExceeded((error) => {
      error.setMessage('Too many attempts — please wait before trying again')
    })
})

/**
 * General API: authenticated users get 100 req/min, guests get 30 req/min.
 */
export const apiThrottle = limiter.define('api', (ctx) => {
  if (ctx.auth.user) {
    return limiter
      .allowRequests(100)
      .every('1 minute')
      .usingKey(`api_user_${ctx.auth.user.id}`)
  }

  return limiter
    .allowRequests(30)
    .every('1 minute')
    .usingKey(`api_ip_${ctx.request.ip()}`)
})
