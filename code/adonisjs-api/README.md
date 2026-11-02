# JIONI Digital — Backend API

AdonisJS v7 REST API for the JIONI Digital platform — Tanzania's premier festival and events platform.

## Quick Start

```bash
# From the monorepo root
pnpm install

# Start services (PostgreSQL, Redis, MinIO, Mailpit)
docker compose up -d

# Copy and configure environment
cp .env.example .env
# Edit .env — set APP_KEY (see below)

# Generate app key
node ace generate:key

# Run migrations
node ace migration:run

# Start dev server with HMR
node ace serve --hmr
```

## Environment Variables

See `.env.example` for the full list. Required:

| Variable | Description |
|----------|-------------|
| `APP_KEY` | 32-char secret — generate with `node ace generate:key` |
| `PG_HOST` / `PG_USER` / `PG_PASSWORD` / `PG_DB_NAME` | PostgreSQL connection |
| `REDIS_HOST` / `REDIS_PORT` | Redis connection |
| `CORS_ORIGIN` | Comma-separated allowed origins (e.g. `http://localhost:5173`) |

Optional:

| Variable | Default | Description |
|----------|---------|-------------|
| `DRIVE_DISK` | `fs` | `fs` (local) or `s3` (production) |
| `QUEUE_DRIVER` | `redis` | `redis`, `database`, or `sync` |
| `LIMITER_STORE` | `database` | `redis`, `database`, or `memory` |
| `FRONTEND_URL` | — | Used in CORS origin check |

## API Reference

Full OpenAPI spec available at `GET /docs.json` (or `/docs.yaml`) when the server is running.

### Auth

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/v1/auth/register` | — | Register (email + password) |
| POST | `/api/v1/auth/login` | — | Login → token or cookie |
| POST | `/api/v1/auth/refresh` | Bearer | Rotate access token |
| POST | `/api/v1/auth/logout` | Bearer | Revoke token |
| POST | `/api/v1/auth/password/reset/request` | — | Request OTP reset email |
| POST | `/api/v1/auth/password/reset/confirm` | — | Verify OTP + set new password |
| POST | `/api/v1/auth/otp/request` | — | Send magic-code email |
| POST | `/api/v1/auth/otp/verify` | — | Verify magic code → token |
| GET | `/api/v1/auth/email/verify/:token` | — | Verify email address |
| POST | `/api/v1/auth/email/verify/resend` | Bearer | Resend verification email |

### Users

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/users/me` | Bearer | Get own profile |
| PATCH | `/api/v1/users/me` | Bearer | Update name / phone |
| POST | `/api/v1/users/me/password` | Bearer | Change password |

### Events

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/events` | — | List events (cursor-paginated) |
| GET | `/api/v1/events/:id` | — | Event detail |
| GET | `/api/v1/events/:id/tickets` | — | Available ticket types |
| POST | `/api/v1/events/:id/rsvp` | Bearer | RSVP |
| DELETE | `/api/v1/events/:id/rsvp` | Bearer | Cancel RSVP |

### Orders & Tickets

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/api/v1/orders` | Bearer | Create order |
| GET | `/api/v1/orders/:id` | Bearer | Order detail |
| POST | `/api/v1/orders/:id/cancel` | Bearer | Cancel order |
| GET | `/api/v1/tickets/mine` | Bearer | My tickets (cursor-paginated) |
| GET | `/api/v1/tickets/:id` | Bearer | Ticket detail + QR secret |
| POST | `/api/v1/tickets/scan` | Admin | Scan & validate ticket |

### Content

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/content` | — | Content archive (cursor-paginated) |
| GET | `/api/v1/content/:id` | — | Content detail |
| POST | `/api/v1/content/:id/like` | Bearer | Toggle like |

### Points & Rewards

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/points/balance` | Bearer | Points balance |
| GET | `/api/v1/points/ledger` | Bearer | Transaction history |
| GET | `/api/v1/rewards` | — | Active rewards catalog |
| POST | `/api/v1/rewards/:id/redeem` | Bearer | Redeem reward |

### Admin

All admin endpoints require `Bearer` token + `role === 'admin'`.

| Method | Path | Description |
|--------|------|-------------|
| GET/POST | `/api/v1/events` + PATCH/DELETE `/:id` | Events CRUD |
| GET | `/api/v1/events/:id/attendees` | Attendee list |
| GET | `/api/v1/events/:id/attendees.csv` | Export CSV |
| GET/POST/PATCH/DELETE | `/api/v1/content` | Content CRUD |
| POST | `/api/v1/content/upload` | File upload |
| GET/POST/PATCH/DELETE | `/api/v1/users` | Users CRUD |
| GET/POST/PATCH/DELETE | `/api/v1/rewards` | Rewards CRUD |

## Authentication Strategy

The API uses a **dual-token strategy** controlled by the `?token=bearer` query parameter:

- **Default (no param):** Sets an `httpOnly` cookie — intended for web browsers.
- **`?token=bearer`:** Returns the token in the JSON response body — intended for mobile/Flutter clients.

## Response Envelope

Every response uses a consistent JSON envelope:

```json
{ "ok": true, "data": { ... }, "meta": { "requestId": "req_01..." } }
{ "ok": false, "error": { "message": "...", "errors": [...] } }
```

## Development Commands

```bash
node ace generate:key          # Generate APP_KEY
node ace migration:run         # Run pending migrations
node ace migration:rollback    # Rollback last batch
node ace migration:status      # Show migration status
node ace make:migration name   # Create migration
node ace make:controller name  # Create controller
node ace test                  # Run tests
node ace build                 # Build for production
node ace serve --hmr           # Dev server with hot reload
```

## Stack

- **Framework:** AdonisJS v7
- **Database:** PostgreSQL (Lucid ORM)
- **Cache / Queue:** Redis (`@adonisjs/queue`)
- **Storage:** AdonisJS Drive (local FS / S3)
- **Email:** `@adonisjs/mail` + Mailpit (dev)
- **Validation:** VineJS
- **Auth:** Access tokens (`DbAccessTokensProvider`)
- **Rate limiting:** `@adonisjs/limiter`
