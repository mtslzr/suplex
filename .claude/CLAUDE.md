# Suplex - Claude Context

## What is this project?
Suplex is a personal pro wrestling tracker. Local-first web app with a Rust
backend (Axum + async-graphql) and React/TypeScript frontend, backed by
PostgreSQL, all running in Docker. Data comes from cagematch.net via an
on-demand scraper.

## Key Documents
- `docs/DESIGN.md` - Architecture, data model, milestones
- GitHub Issues + Milestones - Build plan and task tracking (source of truth)
- GitHub Project `suplex` - Kanban board linked to the repo

## Tech Stack
- **Backend:** Rust, Axum, async-graphql, SeaORM
- **Database:** PostgreSQL 18, SeaORM migrations
- **Frontend:** React 19, Vite, TypeScript, Tailwind CSS
- **GraphQL Client:** Apollo Client, graphql-codegen for types
- **API:** GraphQL at /graphql, REST only for health + scrape triggers
- **External data:** cagematch.net (HTML scrape, rate-limited, manual trigger)
- **Infra:** Docker Compose, GitHub Actions CI

## Ports
- API: `8083`
- Web: `3003`
- Postgres: `5435:5432` (side-by-side with blathers 5432, slate 5433)

## Project Structure
- `api/` - Rust backend (entities, graphql resolvers, services, migrations)
- `web/` - React frontend (pages, components, graphql queries, generated types)
- `docker/` - Dockerfiles and nginx config
- `docs/` - Design docs
- `.claude/` - Claude context files

## Conventions

### Git
- Branches: `feat-xyz`, `fix-xyz`, `chore-xyz`
- Commits: `type(scope): message` (e.g. `feat(api): add promotions query`)
- Types: feat, fix, chore, refactor, test, docs, style
- Scopes: api, web, db, ci, scrape, docs
- Frequent, small commits preferred
- Single-line commit messages, no co-author tags
- All work goes through PRs (main is protected)

### Code
- Comments: simple and descriptive, not conversational. Explain *why*, not *what*.
- Rust: rustfmt + clippy clean (`-D warnings` in CI)
- TypeScript: ESLint + Prettier, strict mode
- SQL: lowercase keywords, snake_case columns

### Testing
- Backend: `cargo test` (unit + integration with Postgres service container)
- Frontend: vitest + testing-library
- CI: GitHub Actions on changes to api/** and web/**

### Scraping
- cagematch.net only
- Rate-limited (default 1 req/s, `SCRAPER_RATE_LIMIT_MS` env var)
- Descriptive `User-Agent` identifying the app
- Manual invocation only — never on app startup
- Store structured data only; no HTML mirror

## Current Phase
v0.1.0 - Foundation

## Owner Preferences
- Likes frequent small commits over large ones
- Prefers feature branches for all work
- Wants code comments but simple, not chatty
- Data portability is important - `make backup` / `make restore` must work

## GitHub Project automation
The `suplex` GitHub Project (Projects v2) auto-adds new issues and PRs,
and moves items across the Kanban board based on state. These workflows
are configured in the Project UI, not in `.github/workflows/`.
