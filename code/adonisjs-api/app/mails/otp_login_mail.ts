import { BaseMail } from '@adonisjs/mail'

export class OtpLoginMail extends BaseMail {
  subject = 'Your JIONI login code'

  constructor(
    private readonly email: string,
    private readonly name: string | null,
    private readonly otp: string
  ) {
    super()
  }

  prepare() {
    const greeting = this.name ? `Hi ${this.name},` : 'Hi,'

    this.message
      .to(this.email)
      .html(
        `<p>${greeting}</p>` +
          `<p>Your JIONI one-time login code is:</p>` +
          `<h2 style="letter-spacing:0.2em;font-size:2em">${this.otp}</h2>` +
          `<p>This code expires in <strong>10 minutes</strong>.</p>` +
          `<p>If you did not request this, you can safely ignore this email.</p>`
      )
      .text(
        `${greeting}\n\nYour JIONI login code: ${this.otp}\n\nExpires in 10 minutes.\n\nIf you did not request this, ignore this email.`
      )
  }
}
