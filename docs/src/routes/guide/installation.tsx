import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/guide/installation')({
  component: InstallationGuide,
})

function InstallationGuide() {
  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      <article className="prose dark:prose-invert max-w-none">
        <h1>Installation</h1>
        <p className="lead">
          Get started with the rok ecosystem in minutes.
        </p>

        <h2>Prerequisites</h2>
        <ul>
          <li>Rust 1.70 or later</li>
          <li>Node.js 18+ (for documentation)</li>
          <li>pnpm (for documentation development)</li>
        </ul>

        <h2>Install rok CLI</h2>
        <pre><code>cargo install rok-cli</code></pre>

        <h2>Verify Installation</h2>
        <pre><code>rok --version</code></pre>
        <p>Should output: <code>rok 0.10.0</code> (or later)</p>

        <h2>Install Development Tools</h2>
        <pre><code># All development tools
cargo install rok-cli rok-gen-model rok-gen-api

# Or install individually as needed
cargo install rok-cli
cargo install rok-gen-model</code></pre>

        <h2>Add Runtime Dependencies</h2>
        <pre><code># For a new project
cargo add rok-orm rok-config rok-utils

# For testing
cargo add rok-test --dev</code></pre>

        <h2>Platform-Specific Notes</h2>

        <h3>Windows</h3>
        <p>
          Ensure you have the Microsoft C++ Build Tools installed.
          Download from: <a href="https://visualstudio.microsoft.com/visual-cpp-build-tools/">Visual Studio Build Tools</a>
        </p>

        <h3>macOS</h3>
        <p>
          Make sure Xcode Command Line Tools are installed:
        </p>
        <pre><code>xcode-select --install</code></pre>

        <h3>Linux</h3>
        <p>
          Install required dependencies:
        </p>
        <pre><code># Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# Fedora
sudo dnf install pkg-config openssl-devel

# Arch
sudo pacman -S pkg-config openssl</code></pre>

        <h2>Next Steps</h2>
        <ul>
          <li><a href="/guide/quickstart">Quick Start Guide</a></li>
          <li><a href="/crates/rok-cli">rok-cli Documentation</a></li>
          <li><a href="/examples">Example Projects</a></li>
        </ul>
      </article>
    </div>
  )
}
