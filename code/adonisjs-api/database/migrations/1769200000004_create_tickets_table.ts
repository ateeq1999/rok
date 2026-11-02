import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'tickets'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('order_id').notNullable().references('id').inTable('orders').onDelete('RESTRICT')
      table.string('ticket_type_id').notNullable().references('id').inTable('ticket_types').onDelete('RESTRICT')
      table.string('event_id').notNullable().references('id').inTable('events').onDelete('RESTRICT')
      table.string('user_id').notNullable().references('id').inTable('users').onDelete('RESTRICT')
      table.string('qr_secret').notNullable()
      table.string('status').notNullable().defaultTo('issued')
      table.timestamp('issued_at').notNullable()
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()

      table.index(['user_id', 'status'])
      table.index(['event_id', 'status'])
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
