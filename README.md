# rok - Run One, Know All

An AI coding agent task runner that collapses multi-step operations into a single JSON invocation.

## Installation

### From Source

```bash
# Clone and build
git clone https://github.com/ateeq1999/rok.git
cd rok
cargo build --release

# Install globally
cargo install --path .
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/ateeq1999/rok/releases).

### Local Development Setup

```bash
# Add to PATH temporarily (Windows PowerShell)
$env:PATH += ";D:\Ateeg\Tools\rok\target\release"

# Or add permanently (Windows PowerShell)
[System.Environment]::SetEnvironmentVariable(
    "PATH",
    [System.Environment]::GetEnvironmentVariable("PATH","User") + ";D:\Ateeg\Tools\rok\target\release",
    "User"
)

# Verify
rok --version
rok --help
```

## Usage

```bash
# JSON inline
rok --json '{ "steps": [...] }'

# From file
rok --file ./task.json

# Pipe from stdin
echo '{ "steps": [...] }' | rok

# Pretty output
rok --json '...' --output pretty

# Dry run
rok --json '...' --dry-run
```

## JSON Payload

```json
{
  "options": {
    "cwd": ".",
    "stopOnError": true,
    "timeoutMs": 30000,
    "env": {}
  },
  "steps": [
    { "type": "bash", "cmd": "find . -name '*.rs'" },
    { "type": "read", "path": "./src/**/*.rs" },
    { "type": "write", "path": "./output.txt", "content": "..." },
    { "type": "mkdir", "path": "./new-dir" },
    { "type": "mv", "from": "old", "to": "new" },
    { "type": "cp", "from": "src", "to": "dest", "recursive": true },
    { "type": "rm", "path": "./file", "recursive": true },
    { "type": "grep", "pattern": "TODO", "path": ".", "ext": ["rs", "toml"] },
    { "type": "replace", "pattern": "foo", "replacement": "bar", "path": ".", "ext": ["rs"] },
    {
      "type": "parallel",
      "steps": [
        { "type": "mkdir", "path": "dir1" },
        { "type": "mkdir", "path": "dir2" }
      ]
    }
  ]
}
```

## Output

```json
{
  "status": "ok",
  "stepsTotal": 5,
  "stepsOk": 5,
  "stepsFailed": 0,
  "durationMs": 150,
  "results": [...]
}
```

## Template System (v3)

### List Available Templates

```bash
rok templates
```

### Use a Template

```bash
# Use built-in template
echo '{"steps":[{"type":"template","builtin":"react-component","output":"./Button.tsx","vars":{"name":"Button"}}]}' | rok -f -

# Use custom template
echo '{"steps":[{"type":"template","name":"my-template","output":"./file.txt","vars":{"name":"value"}}]}' | rok -f -
```

### Create Custom Templates

Place templates in `.rok/templates/<template-name>/` with a `.rok-template.json` schema file:

```json
{
  "name": "my-template",
  "description": "My custom template",
  "version": "1.0.0",
  "author": "you",
  "tags": ["custom", "template"],
  "output": [
    { "from": "template.txt", "to": "{{name}}.txt" }
  ],
  "props": {
    "name": {
      "type": "string",
      "required": true,
      "description": "The name",
      "example": "myfile"
    }
  }
}
```

### Template Filters

Templates support filters for transforming values:

```text
{{ name | camelcase }}   # helloWorld
{{ name | snakecase }}   # hello_world
{{ name | kebabcase }}   # hello-world
{{ name | pascalcase }}  # HelloWorld
{{ name | pluralize }}   # items
{{ name | singularize }} # item
{{ name | uppercase }}   # HELLO
{{ name | lowercase }}   # hello
{{ name | capitalize }}  # Hello
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All steps completed successfully |
| 1 | One or more steps failed |
| 2 | Invalid JSON payload |
| 3 | Startup error |

## License

MIT
