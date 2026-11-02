import { BaseMail } from '@adonisjs/mail'

const FRONTEND_URL = process.env.FRONTEND_URL ?? 'http://localhost:3000'

export class OrderConfirmationMail extends BaseMail {
  subject = 'Your JIONI order confirmation'

  constructor(
    private readonly email: string,
    private readonly name: string | null,
    private readonly orderId: string,
    private readonly eventTitle: string,
    private readonly ticketCount: number,
    private readonly total: number
  ) {
    super()
  }

  prepare() {
    const greeting = this.name ? `Hi ${this.name},` : 'Hi,'
    const ticketsUrl = `${FRONTEND_URL}/tickets`

    this.message
      .to(this.email)
      .html(
        `<p>${greeting}</p>` +
          `<p>Your order for <strong>${this.eventTitle}</strong> is confirmed.</p>` +
          `<table style="border-collapse:collapse;width:100%;max-width:400px">` +
          `<tr><td style="padding:8px 0;color:#666">Order ID</td><td style="padding:8px 0"><code>${this.orderId}</code></td></tr>` +
          `<tr><td style="padding:8px 0;color:#666">Tickets</td><td style="padding:8px 0">${this.ticketCount}</td></tr>` +
          `<tr><td style="padding:8px 0;color:#666">Total</td><td style="padding:8px 0"><strong>KES ${this.total.toLocaleString()}</strong></td></tr>` +
          `</table>` +
          `<p><a href="${ticketsUrl}" style="background:#f5c518;color:#000;padding:10px 20px;border-radius:4px;text-decoration:none;font-weight:bold">View My Tickets</a></p>`
      )
      .text(
        `${greeting}\n\nYour order for "${this.eventTitle}" is confirmed.\n` +
          `Order ID: ${this.orderId}\nTickets: ${this.ticketCount}\nTotal: KES ${this.total.toLocaleString()}\n\n` +
          `View your tickets at ${ticketsUrl}`
      )
  }
}
