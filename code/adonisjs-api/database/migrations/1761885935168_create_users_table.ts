import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'users'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('full_name').nullable()
      table.string('email', 254).notNullable().unique()
      table.string('phone').nullable().unique()
      table.string('password').notNullable()
      table.boolean('email_verified').nullable()
      table.timestamp('email_verified_at').nullable()
      table.boolean('phone_verified').nullable()
      table.timestamp('phone_verified_at').nullable()
      table.integer('points').nullable()
      table.string('role').nullable()
      table.boolean('marketing_consent').nullable()
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
