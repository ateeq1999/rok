import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-cli')({
  component: RokCliDocs,
})

function RokCliDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-cli</h1>
        <p className="text-lg text-muted">
          Run One, Know All — Execute multi-step tasks from JSON
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-cli"
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-1 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
            </svg>
            v0.10.0
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
        <pre><code>cargo install rok-cli</code></pre>

        <h2>Quick Start</h2>
        <h3>Run from Inline JSON</h3>
        <pre><code>{`rok -j '{"steps":[{"type":"bash","cmd":"echo hello"}]}'`}</code></pre>

        <h3>Run from File</h3>
        <pre><code>rok -f task.json</code></pre>

        <h3>Run Saved Task</h3>
        <pre><code>rok run my-task</code></pre>

        <h2>Step Types</h2>
        <div className="grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">File Operations</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>bash</code> — Run shell commands</li>
              <li><code>read</code> — Read files</li>
              <li><code>write</code> — Write files</li>
              <li><code>patch</code> — Surgical edits</li>
              <li><code>mv</code>, <code>cp</code>, <code>rm</code>, <code>mkdir</code></li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">Code Intelligence</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>scan</code> — Project mapping</li>
              <li><code>summarize</code> — File structure</li>
              <li><code>grep</code> — Pattern search</li>
              <li><code>replace</code> — Find/replace</li>
              <li><code>refactor</code> — Rename symbols</li>
              <li><code>deps</code> — Dependency graph</li>
              <li><code>dead_code</code> — Detect unused code</li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">Control Flow</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>if</code> — Conditional execution</li>
              <li><code>each</code> — Loops</li>
              <li><code>parallel</code> — Concurrent execution</li>
            </ul>
          </div>
          <div className="rounded-lg border p-4">
            <h3 className="font-semibold">Maintenance</h3>
            <ul className="mt-2 list-inside list-disc text-sm text-muted-foreground">
              <li><code>boilerplate</code> — Add headers/licenses</li>
              <li><code>import</code> — Manage imports</li>
              <li><code>lint</code> — Run linter</li>
              <li><code>snapshot</code> / <code>restore</code></li>
            </ul>
          </div>
        </div>

        <h2>Example: Multi-Step Task</h2>
        <pre><code>{`{
  "steps": [
    {
      "type": "snapshot",
      "path": ".",
      "snapshot_id": "before-change"
    },
    {
      "type": "refactor",
      "symbol": "oldFunction",
      "rename_to": "newFunction",
      "path": "./src"
    },
    {
      "type": "bash",
      "cmd": "cargo test"
    },
    {
      "type": "if",
      "condition": { "type": "stepFailed", "ref": 2 },
      "then": [
        { "type": "restore", "snapshot_id": "before-change" }
      ]
    }
  ]
}`}</code></pre>

        <h2>CLI Commands</h2>
        <table className="w-full">
          <thead>
            <tr className="border-b">
              <th className="py-2 text-left">Command</th>
              <th className="py-2 text-left">Description</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok -f {'<file>'}</td>
              <td className="py-2 text-sm">Run task from JSON file</td>
            </tr>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok -j {'<json>'}</td>
              <td className="py-2 text-sm">Run task from inline JSON</td>
            </tr>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok run {'<name>'}</td>
              <td className="py-2 text-sm">Run saved task</td>
            </tr>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok save {'<name>'}</td>
              <td className="py-2 text-sm">Save current task</td>
            </tr>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok list</td>
              <td className="py-2 text-sm">List saved tasks</td>
            </tr>
            <tr className="border-b">
              <td className="py-2 font-mono text-sm">rok cache --stats</td>
              <td className="py-2 text-sm">Show cache statistics</td>
            </tr>
          </tbody>
        </table>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-cli" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
          <li><a href="/guide/installation">Installation Guide</a></li>
          <li><a href="/crates/rok-orm">rok-orm</a></li>
          <li><a href="/crates/rok-http">rok-http</a></li>
        </ul>
      </article>
    </div>
  )
}
