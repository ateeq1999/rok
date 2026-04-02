import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-http')({
  component: RokHttpDocs,
})

function RokHttpDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-http</h1>
        <p className="text-lg text-muted">
          Axum-based HTTP layer with middleware stack
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-http"
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
        <pre><code>cargo add rok-http</code></pre>

        <h2>App Builder</h2>
        <p>
          <code>rok_http::App</code> wraps Axum with a fluent builder that wires up the full middleware stack before binding.
        </p>
        <pre><code>{`use rok_http::App;

let app = App::new()
    .route("/health", axum::routing::get(health_handler))
    .router(api_router())          // merge an existing Axum Router
    .with_auth(auth_layer)         // attach AuthLayer (see below)
    .build();                      // returns axum::Router

app.serve("0.0.0.0:3000").await?; // bind and start listening`}</code></pre>

        <h3>Builder Methods</h3>
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
                <td className="py-2 font-mono text-sm">new()</td>
                <td className="py-2 text-sm">Create a new App builder</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">route(path, handler)</td>
                <td className="py-2 text-sm">Register a single route and handler</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">router(router)</td>
                <td className="py-2 text-sm">Merge an existing <code>axum::Router</code></td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">with_auth(layer)</td>
                <td className="py-2 text-sm">Attach an <code>AuthLayer</code> for JWT guard middleware</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">build()</td>
                <td className="py-2 text-sm">Finalize and return the composed <code>axum::Router</code></td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">serve(addr)</td>
                <td className="py-2 text-sm">Bind to the address and start the server</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>Built-in Middleware</h2>
        <div className="grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">Always Active</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>compression</code> — gzip/br response compression</li>
              <li><code>CORS</code> — configurable cross-origin headers</li>
              <li><code>tracing</code> — structured request/response logging</li>
              <li><code>request-id</code> — injects <code>x-request-id</code> header</li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">Optional</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>auth guard</code> — JWT validation via <code>AuthLayer</code></li>
            </ul>
          </div>
        </div>

        <h2>AuthLayer Usage</h2>
        <p>
          <code>AuthLayer</code> integrates with <code>rok-auth</code> to protect routes. It extracts the Bearer token from the <code>Authorization</code> header, verifies it, and injects <code>SessionToken</code> as a request extension.
        </p>
        <pre><code>{`use rok_auth::{Auth, AuthConfig};
use rok_http::{App, AuthLayer};

let auth = Auth::new(AuthConfig {
    secret: "secret".to_string(),
    token_ttl: 3600,
    refresh_ttl: 604800,
    issuer: None,
});

let auth_layer = AuthLayer::new(auth);

let app = App::new()
    .router(protected_routes())
    .with_auth(auth_layer)
    .build();`}</code></pre>

        <p>Inside a protected handler, extract the session claims via Axum's <code>Extension</code> extractor:</p>
        <pre><code>{`use axum::Extension;
use rok_auth::SessionToken;

async fn protected(Extension(session): Extension<SessionToken>) -> String {
    format!("Hello, {}", session.sub)
}`}</code></pre>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-http" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
          <li><a href="/crates/rok-auth">rok-auth — JWT authentication primitives</a></li>
        </ul>
      </article>
    </div>
  )
}
