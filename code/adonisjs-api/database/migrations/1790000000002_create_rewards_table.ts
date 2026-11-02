import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'rewards'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('name').notNullable()
      table.text('description').nullable()
      table.integer('points_cost').notNullable()
      table.integer('quantity_total').nullable()
      table.integer('quantity_redeemed').notNullable().defaultTo(0)
      table.string('status').notNullable().defaultTo('active')
      table.timestamp('expires_at').nullable()
      table.timestamp('created_at').notNullable().defaultTo(this.now())
      table.timestamp('updated_at').nullable()
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
