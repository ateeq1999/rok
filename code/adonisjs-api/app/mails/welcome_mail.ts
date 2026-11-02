import { BaseMail } from '@adonisjs/mail'

const FRONTEND_URL = process.env.FRONTEND_URL ?? 'http://localhost:3000'

export class WelcomeMail extends BaseMail {
  subject = 'Welcome to JIONI Digital'

  constructor(
    private readonly email: string,
    private readonly name: string | null
  ) {
    super()
  }

  prepare() {
    const greeting = this.name ? `Hi ${this.name},` : 'Hi,'

    this.message
      .to(this.email)
      .html(
        `<p>${greeting}</p>` +
          `<p>Welcome to <strong>JIONI Digital</strong> — your home for exclusive events, content, and community.</p>` +
          `<p>Here's what you can do now:</p>` +
          `<ul>
            <li>Browse and RSVP to upcoming events</li>
            <li>Access exclusive member content</li>
            <li>Earn points and redeem rewards</li>
          </ul>` +
          `<p><a href="${FRONTEND_URL}/dashboard" style="background:#f5c518;color:#000;padding:10px 20px;border-radius:4px;text-decoration:none;font-weight:bold">Go to Dashboard</a></p>`
      )
      .text(
        `${greeting}\n\nWelcome to JIONI Digital!\n\nBrowse events, access content, and earn rewards at ${FRONTEND_URL}/dashboard`
      )
  }
}
