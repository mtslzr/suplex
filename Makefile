.PHONY: backup build build-api build-web clean dev dev-api dev-db dev-web down env fmt help install-web lint restore run test test-api test-web up

## Copy .env.example to .env if missing (no-op otherwise)
env:
	@test -f .env || (cp .env.example .env && echo "Created .env from .env.example")

## Rebuild and launch everything in Docker (localhost:3003)
run: env
	docker compose build
	docker compose up

## Launch existing containers without rebuilding
up: env
	docker compose up

## Rebuild only the API image
build-api:
	docker compose build api

## Rebuild only the web image
build-web:
	docker compose build web

## Hot-reload dev (Postgres in Docker, API + web native)
dev: dev-db
	@echo "Postgres running on :5435"
	@echo "Starting API on :8083 and Web on :3003..."
	@cd api && DATABASE_URL=postgres://suplex:suplex@localhost:5435/suplex cargo run &
	@cd web && npm run dev &
	@wait

## Start only Postgres in Docker
dev-db:
	docker compose up -d db

## Run API locally (requires Postgres)
dev-api:
	cd api && DATABASE_URL=postgres://suplex:suplex@localhost:5435/suplex cargo run

## Run web dev server locally
dev-web:
	cd web && npm run dev

## Build all Docker containers without starting
build:
	docker compose build

## Stop all containers
down:
	docker compose down

## Remove Docker containers and volumes
clean:
	docker compose down -v

## Back up database and logos to ./backups/
backup:
	@mkdir -p backups
	docker compose exec db pg_dump -U suplex suplex > backups/suplex-backup.sql
	@rm -rf backups/logos
	docker cp $$(docker compose ps -q api):/app/data/logos backups/logos
	@cp .env backups/.env.backup 2>/dev/null || true
	@echo "Backup saved to ./backups/"

## Restore database and logos from ./backups/
restore:
	@test -f backups/suplex-backup.sql || (echo "No backup found at backups/suplex-backup.sql" && exit 1)
	docker compose up -d db
	@sleep 3
	docker compose exec -T db psql -U suplex suplex < backups/suplex-backup.sql
	@if [ -d backups/logos ]; then docker cp backups/logos/. $$(docker compose ps -q api):/app/data/logos/; echo "Logos restored"; fi
	@echo "Restore complete. Run 'make run' to start."

## Format code
fmt:
	cd api && cargo fmt --all
	cd web && npx prettier --write src/

## Show this help
help:
	@grep -E '^## ' Makefile | sed 's/^## //' | paste - - | column -t -s '	'

## Install web dependencies
install-web:
	cd web && npm install

## Run linters
lint:
	cd api && cargo fmt --all --check && cargo clippy
	cd web && npx eslint . && npx tsc --noEmit

## Run all tests
test: test-api test-web

## Run backend tests
test-api:
	cd api && cargo test -- --test-threads=1

## Run frontend tests
test-web:
	cd web && npx vitest run
