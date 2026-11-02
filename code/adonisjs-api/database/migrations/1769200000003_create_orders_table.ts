import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  async up() {
    this.schema.createTable('orders', (table) => {
      table.string('id').primary()
      table.string('user_id').notNullable().references('id').inTable('users').onDelete('RESTRICT')
      table.string('event_id').notNullable().references('id').inTable('events').onDelete('RESTRICT')
      table.string('status').notNullable().defaultTo('pending_payment')
      table.integer('subtotal_amount').notNullable()
      table.integer('discount_amount').nullable().defaultTo(0)
      table.integer('total_amount').notNullable()
      table.string('currency').notNullable().defaultTo('TZS')
      table.string('idempotency_key').nullable().unique()
      table.timestamp('paid_at').nullable()
      table.timestamp('expires_at').notNullable()
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()

      table.index(['user_id', 'created_at'])
      table.index(['event_id', 'status'])
    })

    this.schema.createTable('order_items', (table) => {
      table.string('id').primary()
      table.string('order_id').notNullable().references('id').inTable('orders').onDelete('CASCADE')
      table.string('ticket_type_id').notNullable().references('id').inTable('ticket_types').onDelete('RESTRICT')
      table.integer('quantity').notNullable()
      table.integer('unit_price').notNullable()
    })
  }

  async down() {
    this.schema.dropTable('order_items')
    this.schema.dropTable('orders')
  }
}
