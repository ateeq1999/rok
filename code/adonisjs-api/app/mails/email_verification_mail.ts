import { BaseMail } from '@adonisjs/mail'

const FRONTEND_URL = process.env.FRONTEND_URL ?? 'http://localhost:3000'

export class EmailVerificationMail extends BaseMail {
  subject = 'Verify your JIONI email address'

  constructor(
    private readonly email: string,
    private readonly name: string | null,
    private readonly token: string
  ) {
    super()
  }

  prepare() {
    const greeting = this.name ? `Hi ${this.name},` : 'Hi,'
    const link = `${FRONTEND_URL}/verify-email?token=${this.token}`

    this.message
      .to(this.email)
      .html(
        `<p>${greeting}</p>` +
          `<p>Please verify your email address to activate your JIONI account.</p>` +
          `<p><a href="${link}" style="background:#f5c518;color:#000;padding:10px 20px;border-radius:4px;text-decoration:none;font-weight:bold">Verify Email</a></p>` +
          `<p>Or copy this link: <code>${link}</code></p>` +
          `<p>This link expires in <strong>24 hours</strong>.</p>`
      )
      .text(
        `${greeting}\n\nVerify your JIONI email:\n${link}\n\nExpires in 24 hours.`
      )
  }
}
