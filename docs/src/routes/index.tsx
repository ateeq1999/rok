import { Link, createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({ component: HomePage })

function HomePage() {
  return (
    <main className="page-wrap px-4 pb-8 pt-14">
      <section className="island-shell rise-in relative overflow-hidden rounded-[2rem] px-6 py-10 sm:px-10 sm:py-14">
        <div className="pointer-events-none absolute -left-20 -top-24 h-56 w-56 rounded-full bg-[radial-gradient(circle,rgba(79,184,178,0.32),transparent_66%)]" />
        <div className="pointer-events-none absolute -bottom-20 -right-20 h-56 w-56 rounded-full bg-[radial-gradient(circle,rgba(47,106,74,0.18),transparent_66%)]" />
        <p className="island-kicker mb-3">rok Documentation</p>
        <h1 className="display-title mb-5 max-w-3xl text-4xl leading-[1.02] font-bold tracking-tight text-[var(--sea-ink)] sm:text-6xl">
          Run One. Know All.
        </h1>
        <p className="mb-8 max-w-2xl text-base text-[var(--sea-ink-soft)] sm:text-lg">
          An AI-native execution engine that transforms multi-step coding tasks into a single JSON document.
          One JSON. All Changes.
        </p>
        <div className="flex flex-wrap gap-3">
          <Link
            to="/guide/installation"
            className="rounded-full border border-[rgba(50,143,151,0.3)] bg-[rgba(79,184,178,0.14)] px-5 py-2.5 text-sm font-semibold text-[var(--lagoon-deep)] no-underline transition hover:-translate-y-0.5 hover:bg-[rgba(79,184,178,0.24)]"
          >
            Get Started
          </Link>
          <Link
            to="/crates/rok-cli"
            className="rounded-full border border-[rgba(23,58,64,0.2)] bg-white/50 px-5 py-2.5 text-sm font-semibold text-[var(--sea-ink)] no-underline transition hover:-translate-y-0.5 hover:border-[rgba(23,58,64,0.35)]"
          >
            View Crates
          </Link>
          <a
            href="https://github.com/ateeq1999/rok"
            target="_blank"
            rel="noopener noreferrer"
            className="rounded-full border border-[rgba(23,58,64,0.2)] bg-white/50 px-5 py-2.5 text-sm font-semibold text-[var(--sea-ink)] no-underline transition hover:-translate-y-0.5 hover:border-[rgba(23,58,64,0.35)]"
          >
            GitHub
          </a>
        </div>
      </section>

      <section className="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {[
          [
            '26 Step Types',
            'File ops, search/replace, refactoring, imports, dead code detection, and more.',
          ],
          [
            '90% Token Reduction',
            'Collapse 20+ API calls and 50K tokens into 1 JSON and ~5K tokens.',
          ],
          [
            'Self-Evolving',
            'rok builds, tests, and releases itself using rok.',
          ],
          [
            'Workspace Architecture',
            '12+ independently publishable crates that compose seamlessly.',
          ],
          [
            'Axum 0.8+ Based',
            'Modern web framework with Rustls, HTTP/2, and starter templates.',
          ],
          [
            'Better-Auth Compatible',
            'Full authentication with OAuth, MFA, sessions, and RBAC.',
          ],
        ].map(([title, desc], index) => (
          <article
            key={title}
            className="island-shell feature-card rise-in rounded-2xl p-5"
            style={{ animationDelay: `${index * 90 + 80}ms` }}
          >
            <h2 className="mb-2 text-base font-semibold text-[var(--sea-ink)]">
              {title}
            </h2>
            <p className="m-0 text-sm text-[var(--sea-ink-soft)]">{desc}</p>
          </article>
        ))}
      </section>

      <section className="island-shell mt-8 grid gap-6 md:grid-cols-2">
        <div className="rounded-2xl p-6">
          <p className="island-kicker mb-2">Ecosystem</p>
          <h2 className="mb-4 text-xl font-semibold text-[var(--sea-ink)]">
            Core Crates
          </h2>
          <ul className="m-0 space-y-2 text-sm text-[var(--sea-ink-soft)]">
            <li><Link to="/crates/rok-cli" className="text-primary hover:underline">rok-cli</Link> — Main CLI orchestrator ✅ v0.10.0</li>
            <li><Link to="/crates/rok-orm" className="text-primary hover:underline">rok-orm</Link> — Eloquent-inspired async ORM 📋 v0.1.0</li>
            <li><Link to="/crates/rok-http" className="text-primary hover:underline">rok-http</Link> — Axum 0.8+ web framework 📋 v0.1.0</li>
            <li><Link to="/crates/rok-auth" className="text-primary hover:underline">rok-auth</Link> — Better-auth compatible auth 📋 v0.1.0</li>
          </ul>
        </div>

        <div className="rounded-2xl p-6">
          <p className="island-kicker mb-2">Quick Links</p>
          <h2 className="mb-4 text-xl font-semibold text-[var(--sea-ink)]">
            Resources
          </h2>
          <ul className="m-0 space-y-2 text-sm text-[var(--sea-ink-soft)]">
            <li><Link to="/guide/installation" className="text-primary hover:underline">Installation Guide</Link></li>
            <li><a href="https://crates.io/crates/rok-cli" className="text-primary hover:underline" target="_blank" rel="noreferrer">crates.io</a></li>
            <li><a href="https://github.com/ateeq1999/rok" className="text-primary hover:underline" target="_blank" rel="noreferrer">GitHub Repository</a></li>
            <li><Link to="/api" className="text-primary hover:underline">API Reference</Link></li>
          </ul>
        </div>
      </section>

      <section className="island-shell mt-8 rounded-2xl p-6">
        <p className="island-kicker mb-2">Example</p>
        <h2 className="mb-4 text-xl font-semibold text-[var(--sea-ink)]">
          One JSON. All Changes.
        </h2>
        <pre className="overflow-x-auto rounded-lg bg-muted p-4 text-xs"><code>{`{
  "steps": [
    { "type": "snapshot", "path": ".", "snapshot_id": "v1" },
    { "type": "refactor", "symbol": "old", "rename_to": "new", "path": "./src" },
    { "type": "bash", "cmd": "cargo test" },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 2 },
      "then": [{ "type": "restore", "snapshot_id": "v1" }]
    }
  ]
}`}</code></pre>
      </section>
    </main>
  )
}
