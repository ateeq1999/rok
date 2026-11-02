import { BaseSeeder } from '@adonisjs/lucid/seeders'
import { DateTime } from 'luxon'
import crypto from 'node:crypto'
import User from '#models/user'
import Event from '#models/event'
import TicketType from '#models/ticket_type'
import Order from '#models/order'
import OrderItem from '#models/order_item'
import Ticket from '#models/ticket'

export default class MainSeeder extends BaseSeeder {
  async run() {
    // ── Users ────────────────────────────────────────────────────────────────
    const [admin, member] = await Promise.all([
      User.updateOrCreate(
        { email: 'admin@jioni.co' },
        {
          fullName: 'JIONI Admin',
          password: 'Jioni2026!',
          role: 'admin',
          emailVerified: true,
          emailVerifiedAt: DateTime.now(),
          points: 0,
        }
      ),
      User.updateOrCreate(
        { email: 'member@jioni.co' },
        {
          fullName: 'Amara Osei',
          password: 'Jioni2026!',
          role: 'user',
          emailVerified: true,
          emailVerifiedAt: DateTime.now(),
          points: 250,
        }
      ),
    ])

    console.log(`  ✓ Users: ${admin.email}, ${member.email}`)

    // ── Events ───────────────────────────────────────────────────────────────
    const now = DateTime.now()

    const eventsData = [
      {
        title: 'SESSIONS 07',
        description:
          'The seventh edition of our iconic sunset-to-sunrise Afro House session. From the shores of Kigamboni, we bring you the finest selectors from across the continent as the Indian Ocean meets the bass. Doors open at 17:00 — the last hour begins at sunset.\n\nLineup: Yogo · Mshindo · Sisi Kwa Sisi · TBA',
        location: 'Kigamboni Beach Club',
        city: 'Dar es Salaam',
        country: 'Tanzania',
        address: 'Kigamboni Peninsula, Dar es Salaam',
        status: 'published',
        category: 'Afro House',
        capacity: 350,
        isFeatured: true,
        isPublic: true,
        startsAt: now.plus({ days: 3 }).set({ hour: 17, minute: 0, second: 0 }),
        endsAt: now.plus({ days: 4 }).set({ hour: 2, minute: 0, second: 0 }),
        salesOpensAt: now.minus({ days: 7 }),
        salesClosesAt: now.plus({ days: 3 }).set({ hour: 14, minute: 0, second: 0 }),
        organizerId: admin.id,
        publishedAt: now.minus({ days: 10 }),
        rsvpCount: 84,
      },
      {
        title: 'SESSIONS ZANZIBAR',
        description:
          'JIONI travels to Zanzibar for an open-air Afro Tech session inside the UNESCO-listed walls of Stone Town. As the call to prayer fades and the stars appear, the music begins.\n\nA limited capacity experience — 200 tickets only.',
        location: 'Forodhani Gardens',
        city: 'Zanzibar',
        country: 'Tanzania',
        address: 'Stone Town Waterfront, Zanzibar',
        status: 'published',
        category: 'Afro Tech',
        capacity: 200,
        isFeatured: true,
        isPublic: true,
        startsAt: now.plus({ days: 18 }).set({ hour: 19, minute: 0, second: 0 }),
        endsAt: now.plus({ days: 19 }).set({ hour: 1, minute: 0, second: 0 }),
        salesOpensAt: now.minus({ days: 3 }),
        salesClosesAt: now.plus({ days: 17 }).set({ hour: 18, minute: 0, second: 0 }),
        organizerId: admin.id,
        publishedAt: now.minus({ days: 3 }),
        rsvpCount: 42,
      },
      {
        title: 'DIALOGUES: The Sound of Now',
        description:
          'A curated afternoon of conversations on the business, culture, and future of African electronic music. Featuring producers, label owners, and tastemakers from Tanzania, Kenya, and South Africa.\n\nPanel discussions + live performance. Light refreshments included.',
        location: 'Alliance Française de Dar es Salaam',
        city: 'Dar es Salaam',
        country: 'Tanzania',
        address: 'Upanga Road, Dar es Salaam',
        status: 'published',
        category: 'Dialogues',
        capacity: 120,
        isFeatured: false,
        isPublic: true,
        startsAt: now.plus({ days: 10 }).set({ hour: 14, minute: 0, second: 0 }),
        endsAt: now.plus({ days: 10 }).set({ hour: 20, minute: 0, second: 0 }),
        salesOpensAt: now.minus({ days: 5 }),
        salesClosesAt: now.plus({ days: 10 }).set({ hour: 12, minute: 0, second: 0 }),
        organizerId: admin.id,
        publishedAt: now.minus({ days: 5 }),
        rsvpCount: 31,
      },
      {
        title: 'ACADEMY: DJ Fundamentals — Cohort 03',
        description:
          'A three-day intensive covering the fundamentals of DJing and Afro House production. Limited to 16 participants. Equipment, recordings, and certificate provided.\n\nDay 1: Theory & Ear Training\nDay 2: Hands-on Mixing\nDay 3: Studio Session + Feedback',
        location: 'JIONI Studio',
        city: 'Dar es Salaam',
        country: 'Tanzania',
        address: 'Masaki, Dar es Salaam',
        status: 'published',
        category: 'Workshop',
        capacity: 16,
        isFeatured: false,
        isPublic: true,
        startsAt: now.plus({ days: 25 }).set({ hour: 10, minute: 0, second: 0 }),
        endsAt: now.plus({ days: 27 }).set({ hour: 17, minute: 0, second: 0 }),
        salesOpensAt: now.minus({ days: 14 }),
        salesClosesAt: now.plus({ days: 24 }).set({ hour: 20, minute: 0, second: 0 }),
        organizerId: admin.id,
        publishedAt: now.minus({ days: 14 }),
        rsvpCount: 0,
      },
      {
        title: 'SESSIONS 06',
        description:
          'The sixth edition — a sold-out sunset session on the Masaki Rooftop. Archived.',
        location: 'Masaki Rooftop',
        city: 'Dar es Salaam',
        country: 'Tanzania',
        address: 'Chole Road, Masaki, Dar es Salaam',
        status: 'published',
        category: 'Afro House',
        capacity: 300,
        isFeatured: false,
        isPublic: true,
        startsAt: now.minus({ days: 30 }).set({ hour: 17, minute: 0, second: 0 }),
        endsAt: now.minus({ days: 29 }).set({ hour: 2, minute: 0, second: 0 }),
        organizerId: admin.id,
        publishedAt: now.minus({ days: 45 }),
        rsvpCount: 148,
      },
    ]

    const events: Event[] = []
    for (const data of eventsData) {
      const ev = await Event.updateOrCreate({ title: data.title, city: data.city }, data as any)
      events.push(ev)
    }

    const [sessions07, sessionsZnz, dialogues, academy, sessions06] = events
    console.log(`  ✓ Events: ${events.map((e) => e.title).join(', ')}`)

    // ── Ticket types ─────────────────────────────────────────────────────────
    const ttData: Array<{
      eventId: string
      name: string
      description: string | null
      perks: string | null
      price: number
      currency: string
      quantityTotal: number
      quantitySold: number
      status: string
      minPerOrder: number
      maxPerOrder: number
    }> = [
      // SESSIONS 07
      {
        eventId: sessions07.id,
        name: 'General Admission',
        description: 'Standing access to the main floor',
        perks: null,
        price: 25000,
        currency: 'TZS',
        quantityTotal: 250,
        quantitySold: 180,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 4,
      },
      {
        eventId: sessions07.id,
        name: 'Circle Member',
        description: 'Discounted entry for JIONI Circle members',
        perks: 'Priority entry queue',
        price: 15000,
        currency: 'TZS',
        quantityTotal: 50,
        quantitySold: 44,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 2,
      },
      {
        eventId: sessions07.id,
        name: 'VIP',
        description: 'VIP lounge access + 2 drinks tokens',
        perks: 'VIP lounge, 2 drink tokens, priority entry',
        price: 75000,
        currency: 'TZS',
        quantityTotal: 30,
        quantitySold: 12,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 2,
      },
      // SESSIONS ZANZIBAR
      {
        eventId: sessionsZnz.id,
        name: 'General Admission',
        description: 'Open-air standing access',
        perks: null,
        price: 35000,
        currency: 'TZS',
        quantityTotal: 160,
        quantitySold: 90,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 4,
      },
      {
        eventId: sessionsZnz.id,
        name: 'VIP',
        description: 'Reserved seating + welcome cocktail',
        perks: 'Reserved seating, welcome cocktail',
        price: 100000,
        currency: 'TZS',
        quantityTotal: 20,
        quantitySold: 8,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 2,
      },
      // DIALOGUES
      {
        eventId: dialogues.id,
        name: 'General',
        description: 'Open to all — free entry',
        perks: null,
        price: 0,
        currency: 'TZS',
        quantityTotal: 100,
        quantitySold: 31,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 2,
      },
      {
        eventId: dialogues.id,
        name: 'Circle Member',
        description: 'Reserved front-row seating for Circle members',
        perks: 'Reserved front row, name on programme',
        price: 0,
        currency: 'TZS',
        quantityTotal: 20,
        quantitySold: 14,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 1,
      },
      // ACADEMY
      {
        eventId: academy.id,
        name: 'Workshop Pass',
        description: '3-day DJ Fundamentals cohort. Equipment + certificate included.',
        perks: 'Equipment loan, studio recording, certificate, alumni Discord',
        price: 150000,
        currency: 'TZS',
        quantityTotal: 16,
        quantitySold: 11,
        status: 'active',
        minPerOrder: 1,
        maxPerOrder: 1,
      },
      // SESSIONS 06 (past)
      {
        eventId: sessions06.id,
        name: 'General Admission',
        description: null,
        perks: null,
        price: 20000,
        currency: 'TZS',
        quantityTotal: 300,
        quantitySold: 300,
        status: 'sold_out',
        minPerOrder: 1,
        maxPerOrder: 4,
      },
    ]

    const ticketTypes: TicketType[] = []
    for (const tt of ttData) {
      const existing = await TicketType.query()
        .where('eventId', tt.eventId)
        .where('name', tt.name)
        .first()
      const saved = existing ?? (await TicketType.create(tt as any))
      ticketTypes.push(saved)
    }

    console.log(`  ✓ Ticket types: ${ticketTypes.length} created`)

    // ── Tickets for member user ───────────────────────────────────────────────
    // 1 issued ticket for SESSIONS 07 (General)
    // 1 checked_in ticket for SESSIONS 06 (General)
    const sessions07General = ticketTypes.find(
      (tt) => tt.eventId === sessions07.id && tt.name === 'General Admission'
    )!
    const sessions06General = ticketTypes.find(
      (tt) => tt.eventId === sessions06.id && tt.name === 'General Admission'
    )!

    const existingTickets = await Ticket.query().where('userId', member.id)
    if (existingTickets.length === 0) {
      // Order for SESSIONS 07
      const order07 = await Order.create({
        userId: member.id,
        eventId: sessions07.id,
        status: 'paid',
        subtotalAmount: sessions07General.price ?? 25000,
        totalAmount: sessions07General.price ?? 25000,
        currency: 'TZS',
        paidAt: DateTime.now().minus({ days: 2 }),
        expiresAt: DateTime.now().plus({ days: 1 }),
      })
      await OrderItem.create({
        orderId: order07.id,
        ticketTypeId: sessions07General.id,
        quantity: 1,
        unitPrice: sessions07General.price ?? 25000,
      })
      await Ticket.create({
        orderId: order07.id,
        ticketTypeId: sessions07General.id,
        eventId: sessions07.id,
        userId: member.id,
        qrSecret: crypto.randomBytes(16).toString('hex'),
        status: 'issued',
        issuedAt: DateTime.now().minus({ days: 2 }),
      })

      // Order for SESSIONS 06 (past event)
      const order06 = await Order.create({
        userId: member.id,
        eventId: sessions06.id,
        status: 'paid',
        subtotalAmount: sessions06General.price ?? 20000,
        totalAmount: sessions06General.price ?? 20000,
        currency: 'TZS',
        paidAt: DateTime.now().minus({ days: 32 }),
        expiresAt: DateTime.now().minus({ days: 29 }),
      })
      await OrderItem.create({
        orderId: order06.id,
        ticketTypeId: sessions06General.id,
        quantity: 1,
        unitPrice: sessions06General.price ?? 20000,
      })
      await Ticket.create({
        orderId: order06.id,
        ticketTypeId: sessions06General.id,
        eventId: sessions06.id,
        userId: member.id,
        qrSecret: crypto.randomBytes(16).toString('hex'),
        status: 'checked_in',
        issuedAt: DateTime.now().minus({ days: 32 }),
      })

      console.log(`  ✓ Tickets: 2 tickets issued for ${member.email}`)
    } else {
      console.log(`  ✓ Tickets: skipped (already exist)`)
    }
  }
}
