import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'ticket_types'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('event_id').notNullable().references('id').inTable('events').onDelete('CASCADE')
      table.string('name').notNullable()
      table.text('description').nullable()
      table.text('perks').nullable()
      table.integer('price').nullable()
      table.string('currency').nullable().defaultTo('TZS')
      table.integer('quantity_total').notNullable()
      table.integer('quantity_sold').nullable().defaultTo(0)
      table.integer('min_per_order').nullable().defaultTo(1)
      table.integer('max_per_order').nullable().defaultTo(10)
      table.string('status').nullable().defaultTo('active')
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()

      table.index(['event_id', 'status'])
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
