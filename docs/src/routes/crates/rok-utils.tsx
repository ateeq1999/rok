import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/crates/rok-utils')({
  component: RokUtilsDocs,
})

function RokUtilsDocs() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>rok-utils</h1>
        <p className="text-lg text-muted">
          Shared utility functions for the rok ecosystem
        </p>

        <div className="my-4 flex items-center gap-4">
          <a
            href="https://crates.io/crates/rok-utils"
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
        <pre><code>cargo add rok-utils</code></pre>

        <h2>String Module</h2>
        <p>
          <code>rok_utils::string</code> provides case-conversion helpers used throughout the rok code-generation and ORM layers.
        </p>
        <pre><code>{`use rok_utils::string::{to_snake_case, to_pascal_case, to_camel_case, to_kebab_case};

to_snake_case("MyTableName")   // "my_table_name"
to_pascal_case("my_table")     // "MyTable"
to_camel_case("my_table")      // "myTable"
to_kebab_case("MyTableName")   // "my-table-name"`}</code></pre>

        <div className="rounded-lg border p-4">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="py-2 text-left">Function</th>
                <th className="py-2 text-left">Input</th>
                <th className="py-2 text-left">Output</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">to_snake_case</td>
                <td className="py-2 font-mono text-sm">"MyTableName"</td>
                <td className="py-2 font-mono text-sm">"my_table_name"</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">to_pascal_case</td>
                <td className="py-2 font-mono text-sm">"my_table"</td>
                <td className="py-2 font-mono text-sm">"MyTable"</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">to_camel_case</td>
                <td className="py-2 font-mono text-sm">"my_table"</td>
                <td className="py-2 font-mono text-sm">"myTable"</td>
              </tr>
              <tr className="border-b">
                <td className="py-2 font-mono text-sm">to_kebab_case</td>
                <td className="py-2 font-mono text-sm">"MyTableName"</td>
                <td className="py-2 font-mono text-sm">"my-table-name"</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2>Path Utilities</h2>
        <p>
          Helpers for normalising and joining filesystem paths in a cross-platform manner.
        </p>
        <pre><code>{`use rok_utils::path::{normalize, join, file_stem};

let p = normalize("./src/../src/main.rs");  // "src/main.rs"
let q = join("src", "models/user.rs");       // "src/models/user.rs"
let s = file_stem("src/main.rs");            // "main"`}</code></pre>

        <h2>FS Utilities</h2>
        <p>
          Synchronous filesystem helpers that wrap <code>std::fs</code> with richer error context.
        </p>
        <pre><code>{`use rok_utils::fs::{read_to_string, write_file, ensure_dir};

let contents = read_to_string("config.toml")?;
write_file("output.txt", "hello")?;
ensure_dir("migrations")?;  // creates the directory if it does not exist`}</code></pre>

        <h2>Result Helpers</h2>
        <p>
          Convenience macros and functions for propagating or converting errors in code-generation contexts.
        </p>
        <pre><code>{`use rok_utils::result::{bail, ensure, wrap_err};

// bail! returns early with a formatted error
bail!("unsupported type: {}", type_name);

// ensure! returns early if a condition is false
ensure!(columns.len() > 0, "table must have at least one column");

// wrap_err attaches context to any std::error::Error
let data = std::fs::read_to_string(path).map_err(|e| wrap_err(e, "reading migration file"))?;`}</code></pre>

        <h2>Links</h2>
        <ul>
          <li><a href="https://crates.io/crates/rok-utils" target="_blank" rel="noreferrer">crates.io</a></li>
          <li><a href="https://github.com/ateeq1999/rok" target="_blank" rel="noreferrer">GitHub</a></li>
        </ul>
      </article>
    </div>
  )
}
