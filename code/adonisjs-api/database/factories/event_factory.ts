import factory from '@adonisjs/lucid/factories'
import { DateTime } from 'luxon'
import Event from '#models/event'

const VENUES = [
  { location: 'Kigamboni Beach Club', city: 'Dar es Salaam', address: 'Kigamboni Peninsula' },
  { location: 'Masaki Rooftop', city: 'Dar es Salaam', address: 'Chole Road, Masaki' },
  { location: 'Alliance Française', city: 'Dar es Salaam', address: 'Upanga Road' },
  { location: 'Stone Town Beach', city: 'Zanzibar', address: 'Forodhani Gardens' },
]

const CATEGORIES = ['Afro House', 'Afro Tech', 'Festival', 'Dialogues', 'Workshop']

export const EventFactory = factory
  .define(Event, async ({ faker }) => {
    const venue = faker.helpers.arrayElement(VENUES)
    const startsAt = DateTime.fromJSDate(
      faker.date.between({ from: new Date(), to: new Date(Date.now() + 90 * 86400_000) })
    )
    return {
      title: faker.helpers.arrayElement([
        'SESSIONS',
        'SESSIONS ZANZIBAR',
        'SUNDOWNER',
        'DEEP HOUSE NIGHT',
        'AFRO TECH TAKEOVER',
      ]),
      description: faker.lorem.paragraphs(2),
      location: venue.location,
      city: venue.city,
      country: 'Tanzania',
      address: venue.address,
      status: 'published',
      category: faker.helpers.arrayElement(CATEGORIES),
      capacity: faker.helpers.arrayElement([200, 350, 500]),
      isFeatured: faker.datatype.boolean({ probability: 0.3 }),
      isPublic: true,
      startsAt,
      endsAt: startsAt.plus({ hours: faker.number.int({ min: 4, max: 8 }) }),
    }
  })
  .build()
