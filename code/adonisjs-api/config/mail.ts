import env from '#start/env'
import { defineConfig, transports } from '@adonisjs/mail'

export default defineConfig({
  default: 'mailpit',

  from: {
    address: 'noreply@jioni.co.tz',
    name: 'JIONI Digital',
  },

  mailers: {
    mailpit: transports.smtp({
      host: env.get('MAIL_HOST', 'localhost'),
      port: env.get('MAIL_PORT', 1025),
      secure: false,
      ignoreTLS: true,
    }),
  },
})