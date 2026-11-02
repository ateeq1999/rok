import factory from '@adonisjs/lucid/factories'
import TicketType from '#models/ticket_type'

export const TicketTypeFactory = factory
  .define(TicketType, async ({ faker }) => {
    const price = faker.helpers.arrayElement([0, 15000, 25000, 35000, 50000, 75000])
    return {
      name: faker.helpers.arrayElement(['General Admission', 'Circle Member', 'VIP', 'Early Bird']),
      description: faker.datatype.boolean() ? faker.lorem.sentence() : null,
      price,
      currency: 'TZS',
      quantityTotal: faker.helpers.arrayElement([50, 100, 200, 350]),
      quantitySold: 0,
      status: 'active',
      minPerOrder: 1,
      maxPerOrder: 4,
    }
  })
  .build()
