import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-orm')({
  component: RokOrmDocs,
})

function RokOrmDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-orm</h1>
        <p className="text-lg text-muted">
          Eloquent-inspired ORM with QueryBuilder and async PostgreSQL executor
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-orm"
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
            </svg>
            v0.1.0
          </a>
          <a
            href="https://github.com/ateeq1999/rok"
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
          >
            <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />
            </svg>
            GitHub
          </a>
        </div>

        <h2>Installation</h2>
        <pre><code>cargo add rok-orm</code></pre>
        <p>To use the async PostgreSQL executor, enable the <code>postgres</code> feature:</p>
        <pre><code>cargo add rok-orm --features postgres</code></pre>

        <h2>derive(Model)</h2>
        <p>
          Annotate a struct with <code>#[derive(Model)]</code> to generate table metadata, column mappings, and a <code>QueryBuilder</code> entry-point. The struct fields map directly to database columns.
        </p>
        <pre><code>{`use rok_orm::Model;

#[derive(Debug, Model)]
#[table = "users"]
pub struct User {
    pub id:    i64,
    pub name:  String,
    pub email: String,
}`}</code></pre>

        <h2>QueryBuilder</h2>
        <p>
          <code>QueryBuilder</code> provides a fluent interface for composing SQL <code>SELECT</code> statements. Call <code>to_sql()</code> at any point to obtain the final SQL string and its bound parameters.
        </p>
        <pre><code>{`use rok_orm::QueryBuilder;

let (sql, params) = QueryBuilder::for_model::<User>()
    .where_eq("email", "alice@example.com")
    .where_like("name", "Ali%")
    .order_by_desc("id")
    .limit(10)
    .offset(0)
    .to_sql();

println!("{sql}");
// SELECT * FROM users WHERE email = $1 AND name LIKE $2 ORDER BY id DESC LIMIT 10 OFFSET 0`}</code></pre>

        <h3>QueryBuilder Methods</h3>
        <div className="rounded-lg border p-4">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-2 text-left">Method</th>
                <th className="py-2 text-left">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">where_eq(col, val)</td>
                <td className="py-2 text-sm">Append <code>col = $n</code> condition</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">where_like(col, pat)</td>
                <td className="py-2 text-sm">Append <code>col LIKE $n</code> condition</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">order_by_desc(col)</td>
                <td className="py-2 text-sm">Add <code>ORDER BY col DESC</code> clause</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">limit(n)</td>
                <td className="py-2 text-sm">Set the <code>LIMIT</code> clause</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">offset(n)</td>
                <td className="py-2 text-sm">Set the <code>OFFSET</code> clause</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">to_sql()</td>
                <td className="py-2 text-sm">Return the SQL string and bound parameter list</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>PgModel Trait</h2>
        <p>
          Enable the <code>postgres</code> feature to unlock <code>PgModel</code>. The trait is automatically implemented for every <code>#[derive(Model)]</code> struct and provides async CRUD helpers backed by <code>sqlx</code>.
        </p>
        <pre><code>{`use rok_orm::postgres::PgModel;

let pool = sqlx::PgPool::connect("postgres://user:pass@localhost/mydb").await?;

// Fetch all rows
let users: Vec<User> = User::all(&pool).await?;

// Find by primary key
let user: Option<User> = User::find_by_pk(&pool, 1).await?;

// Insert a new row
let new_user = User { id: 0, name: "Alice".into(), email: "alice@example.com".into() };
let inserted: User = User::create(&pool, new_user).await?;

// Delete by primary key
User::delete_by_pk(&pool, 1).await?;`}</code></pre>

        <h3>PgModel Methods</h3>
        <div className="rounded-lg border p-4">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-2 text-left">Method</th>
                <th className="py-2 text-left">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">all(pool)</td>
                <td className="py-2 text-sm">Fetch every row in the table</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">find_by_pk(pool, id)</td>
                <td className="py-2 text-sm">Fetch a single row by primary key</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">create(pool, model)</td>
                <td className="py-2 text-sm">Insert a new row and return the persisted record</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">delete_by_pk(pool, id)</td>
                <td className="py-2 text-sm">Delete the row with the given primary key</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>Executor Functions</h2>
        <p>
          Low-level helpers for executing arbitrary SQL when the ORM methods are not sufficient.
        </p>
        <pre><code>{`use rok_orm::postgres::{fetch_all, count, execute_raw};

// Fetch rows with a custom query
let users: Vec<User> = fetch_all(&pool, "SELECT * FROM users WHERE active = $1", &[&true]).await?;

// Count rows matching a condition
let total: i64 = count(&pool, "SELECT count(*) FROM users WHERE active = $1", &[&true]).await?;

// Execute a DML/DDL statement
execute_raw(&pool, "TRUNCATE users RESTART IDENTITY", &[]).await?;`}</code></pre>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-orm" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
          <li><a href="/crates/rok-migrate">rok-migrate — SQL migration runner</a></li>
        </ul>
      </article>
    </div>
  )
}
