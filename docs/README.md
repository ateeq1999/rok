# rok Documentation

> **docs.rok.dev** — The official documentation for the rok ecosystem

Built with [TanStack Start](https://tanstack.com/start) — Fast, modern, SSR-ready documentation.

---

## Quick Start

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview
```

---

## Structure

```
docs/
├── content/              ← Markdown/MDX source files
│   ├── guides/           ← User guides
│   ├── crates/           ← Per-crate documentation
│   └── api/              ← API reference
├── src/
│   ├── routes/           ← TanStack Start routes
│   │   ├── __root.tsx    ← Root layout
│   │   ├── index.tsx     ← Home page
│   │   ├── guide/        ← Guide routes
│   │   ├── crates/       ← Crate routes
│   │   └── api/          ← API routes
│   └── components/       ← Reusable components
├── public/               ← Static assets
├── package.json
└── tanstack-start.config.ts
```

---

## Adding New Documentation

### 1. Add Content File

Create a new `.mdx` file in `content/`:

```bash
# New guide
echo "# My Guide" > content/guides/my-guide.mdx

# New crate docs
echo "# rok-orm" > content/crates/rok-orm.mdx
```

### 2. Add Route

Create a corresponding route in `src/routes/`:

```tsx
// src/routes/guide/my-guide.tsx
import { createFileRoute } from '@tanstack/react-router'
import content from '../../content/guides/my-guide.mdx'

export const Route = createFileRoute('/guide/my-guide')({
  component: () => <content />
})
```

### 3. Add to Navigation

Update the navigation component to include your new page.

---

## Writing Documentation

### MDX Syntax

```mdx
# Heading

Regular text with **bold** and *italic*.

## Code Blocks

```rust
use rok_orm::Model;

#[derive(Model)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}
```

## Components

<ApiTable name="Model" methods={['find', 'all', 'save']} />
```

### Frontmatter

```mdx
---
title: Quick Start
description: Get started with rok in 5 minutes
---

# Quick Start
```

---

## Deployment

### Vercel

1. Connect GitHub repository
2. Set build command: `pnpm build`
3. Set output directory: `dist`
4. Deploy!

### Netlify

1. Connect GitHub repository
2. Set build command: `pnpm build`
3. Set publish directory: `dist`
4. Deploy!

---

## Development Guidelines

### Writing Style

- Use **active voice**
- Keep sentences **short and clear**
- Include **code examples** for all features
- Add **type annotations** in Rust code
- Use **bold** for important terms

### Code Examples

```rust
// ✅ Good: Complete, runnable example
use rok_orm::Model;

#[derive(Model)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// Usage
let users = User::all(&pool).await?;
```

```rust
// ❌ Bad: Incomplete, missing context
let users = User::all().await?; // What is User? Where is pool?
```

### Screenshots

- Use PNG format
- Include alt text
- Keep file size under 500KB
- Use consistent styling

---

## Contributing

1. Fork the repository
2. Create a branch: `git checkout -b docs/my-change`
3. Make your changes
4. Preview locally: `pnpm dev`
5. Submit a PR

---

## License

MIT — See [LICENSE](../LICENSE)

---

**rok** — Run One. Know All.
