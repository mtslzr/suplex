# Suplex

A personal pro wrestling tracker for schedules, promotions, and champions.

## What is this?

Suplex is a local-first web app for following pro wrestling across promotions.
Add promotions by their cagematch.net ID, scrape metadata on demand, and
browse upcoming shows, current champions, and rosters — all filterable by
promotion.

### Features (planned)

- **Schedule** - Upcoming and past events across tracked promotions, with filters
- **Promotions** - Recent/upcoming shows, current champions, and roster per promotion
- **Championships** - Current champion per title, how they won, reign length
- **Settings** - Add/remove/toggle promotions by cagematch.net ID
- **Scrape** - Pull data from cagematch.net on demand (rate-limited, manual)

## Tech Stack

| Layer    | Technology                              |
|----------|-----------------------------------------|
| Backend  | Rust, Axum, async-graphql, SeaORM       |
| Frontend | React, Vite, TypeScript, Tailwind CSS   |
| Database | PostgreSQL 18                           |
| Infra    | Docker Compose                          |
| CI       | GitHub Actions                          |

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) (for local development)
- [Node.js 24+](https://nodejs.org/) (for local development)

### Run with Docker

```bash
cp .env.example .env    # configure your settings
make run                # build and start all services
```

- Frontend: http://localhost:3003
- GraphQL Playground: http://localhost:8083/graphiql
- API: http://localhost:8083/graphql

### Local Development

```bash
make dev    # Postgres in Docker, API + web native
```

See [docs/DESIGN.md](docs/DESIGN.md) for architecture details.

## License

Private project.
