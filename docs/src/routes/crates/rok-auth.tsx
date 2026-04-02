import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-auth')({
  component: RokAuthDocs,
})

function RokAuthDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-auth</h1>
        <p className="text-lg text-muted">
          JWT authentication and RBAC for the rok ecosystem
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-auth"
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
        <pre><code>cargo add rok-auth</code></pre>

        <h2>Quick Start</h2>
        <h3>Creating an Auth Instance</h3>
        <pre><code>{`use rok_auth::{Auth, AuthConfig};

let config = AuthConfig {
    secret: "your-secret-key".to_string(),
    token_ttl: 3600,       // access token TTL in seconds
    refresh_ttl: 604800,   // refresh token TTL in seconds
    issuer: Some("my-app".to_string()),
};
let auth = Auth::new(config);`}</code></pre>

        <h3>Signing and Verifying Tokens</h3>
        <pre><code>{`// Sign an access token for a subject
let token = auth.sign("user-123")?;

// Verify and extract the session token claims
let session: SessionToken = auth.verify(&token)?;
println!("{}", session.sub); // "user-123"`}</code></pre>

        <h3>Refresh Token Flow</h3>
        <pre><code>{`// Sign a refresh token
let refresh = auth.sign_refresh("user-123")?;

// Verify the refresh token
let claims: RefreshClaims = auth.verify_refresh(&refresh)?;

// Exchange refresh token for a new access token
let new_token = auth.exchange(&refresh)?;`}</code></pre>

        <h2>AuthConfig Fields</h2>
        <div className="rounded-lg border p-4">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-2 text-left">Field</th>
                <th className="py-2 text-left">Type</th>
                <th className="py-2 text-left">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">secret</td>
                <td className="py-2 font-mono text-sm">String</td>
                <td className="py-2 text-sm">HMAC signing secret for JWT operations</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">token_ttl</td>
                <td className="py-2 font-mono text-sm">u64</td>
                <td className="py-2 text-sm">Access token lifetime in seconds</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">refresh_ttl</td>
                <td className="py-2 font-mono text-sm">u64</td>
                <td className="py-2 text-sm">Refresh token lifetime in seconds</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">issuer</td>
                <td className="py-2 font-mono text-sm">Option&lt;String&gt;</td>
                <td className="py-2 text-sm">Optional <code>iss</code> claim embedded in tokens</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>SessionToken</h2>
        <p>
          Returned by <code>auth.verify()</code>. Contains the decoded claims from an access token.
        </p>
        <pre><code>{`pub struct SessionToken {
    pub sub: String,       // subject (e.g. user ID)
    pub iat: u64,          // issued-at timestamp
    pub exp: u64,          // expiry timestamp
    pub iss: Option<String>,
}`}</code></pre>

        <h2>RefreshClaims</h2>
        <p>
          Returned by <code>auth.verify_refresh()</code>. Carries the subject from a refresh token so the caller can look up the user before issuing a new access token.
        </p>
        <pre><code>{`pub struct RefreshClaims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}`}</code></pre>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-auth" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
          <li><a href="/crates/rok-http">rok-http — integrate auth with the HTTP layer</a></li>
        </ul>
      </article>
    </div>
  )
}
