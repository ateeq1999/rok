import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-migrate')({
  component: RokMigrateDocs,
})

function RokMigrateDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-migrate</h1>
        <p className="text-lg text-muted">
          SQL migration runner with async PostgreSQL execution
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-migrate"
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
        <pre><code>cargo add rok-migrate</code></pre>
        <p>To use the async PostgreSQL runner, enable the <code>postgres</code> feature:</p>
        <pre><code>cargo add rok-migrate --features postgres</code></pre>

        <h2>Migration File Format</h2>
        <p>
          Each migration is a plain <code>.sql</code> file containing an <code>UP</code> and an optional <code>DOWN</code> section separated by marker comments. Files are discovered and sorted by name, so a numeric prefix is recommended.
        </p>
        <pre><code>{`-- UP
CREATE TABLE users (
    id   BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- DOWN
DROP TABLE users;`}</code></pre>

        <h2>Migrator API</h2>
        <p>
          <code>Migrator</code> loads migration files from a directory and lets you inspect or plan runs without touching the database.
        </p>
        <pre><code>{`use rok_migrate::Migrator;

let mut migrator = Migrator::new();
migrator.add("migrations/001_create_users.sql")?;

// Inspect state
let pending   = migrator.pending();    // migrations not yet applied
let applied   = migrator.applied();    // migrations already tracked
let up_plan   = migrator.plan_up();    // ordered list to run UP
let down_plan = migrator.plan_down();  // ordered list to roll back`}</code></pre>

        <h3>Migrator Methods</h3>
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
                <td className="py-2 font-mono text-sm">add(path)</td>
                <td className="py-2 text-sm">Load a migration file into the migrator</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">pending()</td>
                <td className="py-2 text-sm">Return migrations not yet applied</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">applied()</td>
                <td className="py-2 text-sm">Return migrations already recorded in history</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">plan_up()</td>
                <td className="py-2 text-sm">Ordered list of UP statements to execute</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">plan_down()</td>
                <td className="py-2 text-sm">Ordered list of DOWN statements to execute</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>Async PostgreSQL Runner</h2>
        <p>
          Enable the <code>postgres</code> feature to get a <code>PgRunner</code> that applies migrations against a live database and tracks applied versions in a history table.
        </p>
        <pre><code>{`use rok_migrate::postgres::PgRunner;

let runner = PgRunner::connect("postgres://user:pass@localhost/mydb").await?;

// Create the migrations history table if it does not exist
runner.ensure_history_table().await?;

// Query which versions have already been applied
let versions = runner.applied_versions().await?;

// Apply all pending UP migrations
runner.run_up(&migrator).await?;

// Roll back the last applied migration
runner.run_down(&migrator).await?;`}</code></pre>

        <h3>PgRunner Methods</h3>
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
                <td className="py-2 font-mono text-sm">ensure_history_table()</td>
                <td className="py-2 text-sm">Create the schema_migrations table if absent</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">applied_versions()</td>
                <td className="py-2 text-sm">Fetch all version strings recorded in history</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">run_up(migrator)</td>
                <td className="py-2 text-sm">Apply all pending UP migrations in order</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">run_down(migrator)</td>
                <td className="py-2 text-sm">Roll back the most recently applied migration</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-migrate" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
          <li><a href="/crates/rok-orm">rok-orm — ORM and query builder</a></li>
        </ul>
      </article>
    </div>
  )
}
