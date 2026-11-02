import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'contents'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.string('id').primary()
      table.string('title').notNullable()
      table.string('slug').notNullable().unique()
      table.string('type').notNullable().defaultTo('video') // video | article | podcast
      table.string('tier').notNullable().defaultTo('free')  // free | member | premium
      table.text('description').nullable()
      table.text('body').nullable()
      table.string('thumbnail_url').nullable()
      table.string('video_url').nullable()
      table.integer('duration_seconds').nullable()
      table.integer('like_count').notNullable().defaultTo(0)
      table.timestamp('published_at').nullable()
      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()

      table.index(['tier', 'published_at'])
      table.index(['type', 'published_at'])
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
