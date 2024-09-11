.PHONY: up dev

include docker/.env
export $(shell sed 's/=.*//' .env)

docker/up:
	@echo "Starting Docker containers..."
	@cd docker && docker compose up -d

db/migrate/init:
	@export DATABASE_URL="postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_URL):$(DATABASE_PORT)/$(DATABASE_NAME)" && \
	cd backend && \
	sea-orm-cli migrate init && \
	unset DATABASE_URL

db/migrate/up:
	@export DATABASE_URL="postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_URL):$(DATABASE_PORT)/$(DATABASE_NAME)" && \
	cd backend && \
	sea-orm-cli migrate up && \
	unset DATABASE_URL

db/migrate/down:
	@export DATABASE_URL="postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_URL):$(DATABASE_PORT)/$(DATABASE_NAME)" && \
	cd backend && \
	sea-orm-cli migrate down && \
	unset DATABASE_URL

app/back/up:
	@echo "Starting backend..."
	@cd backend && cargo run --bin app

app/front/up:
	@echo "Starting frontend..."
	@cd frontend && pnpm dev
