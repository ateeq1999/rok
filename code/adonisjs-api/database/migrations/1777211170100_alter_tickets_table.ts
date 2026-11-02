import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'tickets'

  async up() {
    this.schema.alterTable(this.tableName, (table) => {
      table.timestamp('checked_in_at').nullable()
      table.string('checked_in_by', 26).nullable().references('users.id')
    })
  }

  async down() {
    this.schema.alterTable(this.tableName, (table) => {
      table.dropColumn('checked_in_at')
      table.dropColumn('checked_in_by')
    })
  }
}