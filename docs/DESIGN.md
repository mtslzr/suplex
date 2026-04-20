# Suplex — Design

## Purpose

Local-first web app for tracking pro wrestling across multiple promotions.
Data is scraped from cagematch.net on demand, stored in Postgres, and served
through a GraphQL API to a React frontend.

## Tech decisions

- **Rust + Axum + async-graphql + SeaORM** — matches Slate; GraphQL fits the
  deeply-nested promotion → titles → champions relationships well.
- **PostgreSQL 18** — relational shape, JSONB for flexible bits.
- **React + Vite + TypeScript + Tailwind** — clean/modern theme, promotion
  colors used as accents only.
- **Docker Compose** — single `make run` starts everything.
- **Ports** — api `8083`, web `3003`, db `5435:5432` (so blathers/slate/
  claymore can run side-by-side).

## Data model (v0.1)

### promotions
User-managed list. Only `cagematch_id` and `nickname` are required. Canonical
fields (name, abbreviation, country, logo) are filled by the scraper.

| column          | type         | notes                                    |
|-----------------|--------------|------------------------------------------|
| id              | uuid         | pk                                       |
| cagematch_id    | int          | unique; the `nr=` in cagematch URLs      |
| nickname        | text         | unique; internal label only              |
| canonical_name  | text         | scraped                                  |
| abbreviation    | text         | scraped                                  |
| country         | text         | scraped                                  |
| logo_url        | text         | scraped                                  |
| cagematch_url   | text         | derived from cagematch_id                |
| accent_color    | text         | user-editable; hex string                |
| enabled         | bool         | toggle to include in next scrape         |
| last_synced_at  | timestamptz  | nullable                                 |
| created_at      | timestamptz  |                                          |
| updated_at      | timestamptz  |                                          |

### sync_log
Every scrape run, per-promotion or whole-library.

| column          | type         | notes                                    |
|-----------------|--------------|------------------------------------------|
| id              | uuid         | pk                                       |
| promotion_id    | uuid         | null for "all promotions" runs           |
| scope           | text         | `promotion` / `titles` / `events` / `all`|
| started_at      | timestamptz  |                                          |
| completed_at    | timestamptz  | nullable                                 |
| status          | text         | running / success / error                |
| items_created   | int          |                                          |
| items_updated   | int          |                                          |
| error           | text         | nullable                                 |

Schema for wrestlers, titles, championships, and events lands in v0.2/v0.3
when the scraper goes in.

## Scraping posture (cagematch.net)

- Rate limited (default 1 req/s, configurable via `SCRAPER_RATE_LIMIT_MS`).
- Descriptive `User-Agent` (`SCRAPER_USER_AGENT`) identifying the app + repo.
- Manual invocation only — no scraping on every app start.
- Store structured data only; do not mirror HTML or images beyond logos.
- If the site objects, pivot to a different source rather than evading.

## Milestones

- **v0.1.0** Foundation — scaffold, CI, Docker, DB, migrations, empty pages
- **v0.2.0** Promotions — Settings page, add/remove/enable by cagematch ID, validation fetch
- **v0.3.0** Scraper — promo metadata + roster + titles + current champions
- **v0.4.0** Schedule — events list with filters (promotion, date range)
- **v0.5.0** Championships page — current champions grid, how won, reign length
- **v0.6.0** Promotion detail page — roster, recent/upcoming shows, champions
- **v0.7.0** Polish — background sync, sync-log UI, export, dark mode

## Non-goals (for now)

- Full title lineage history (too messy/brittle; link out to cagematch instead)
- Match-card details (v0.2+ at earliest)
- Multiple users / auth
- Mobile app
