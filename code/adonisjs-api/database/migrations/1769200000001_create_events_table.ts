import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'events'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('organizer_id').nullable().references('id').inTable('users').onDelete('SET NULL')
      table.string('title').notNullable()
      table.text('description').nullable()
      table.string('cover_image_url').nullable()
      table.string('location').notNullable()
      table.string('city').notNullable()
      table.string('country').nullable()
      table.string('address').nullable()
      table.string('latitude').nullable()
      table.string('longitude').nullable()
      table.string('status').nullable()
      table.string('category').nullable()
      table.integer('capacity').nullable()
      table.integer('rsvp_count').nullable().defaultTo(0)
      table.boolean('is_featured').nullable().defaultTo(false)
      table.boolean('is_public').nullable().defaultTo(true)
      table.timestamp('starts_at').notNullable()
      table.timestamp('ends_at').nullable()
      table.timestamp('sales_opens_at').nullable()
      table.timestamp('sales_closes_at').nullable()
      table.timestamp('published_at').nullable()
      table.timestamp('deleted_at').nullable()
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()

      table.index(['status', 'deleted_at'])
      table.index(['starts_at'])
      table.index(['city'])
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
