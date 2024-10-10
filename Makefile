.PHONY: up dev

include docker/.env
export $(shell sed 's/=.*//' .env)

DATABASE="postgresql://$(DATABASE_USER):$(DATABASE_PASSWORD)@$(DATABASE_URL):$(DATABASE_PORT)/$(DATABASE_NAME)"

docker/up:
	@echo "Starting Docker containers..."
	@cd docker && docker compose up -d

docker/nuke:
	@echo "Starting Docker containers..."
	@cd docker && docker compose down -v

db/exec:
	PGPASSWORD=$(DATABASE_PASSWORD) psql -h $(DATABASE_URL) -U $(DATABASE_USER) $(DATABASE_NAME)

db/migrate/up-verbose:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend && \
		sea-orm-cli migrate -v
	unset DATABASE_URL

db/migrate/init:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend && \
	sea-orm-cli migrate init && \
	unset DATABASE_URL

# Up migration
db/migrate/up:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- up && \
	unset DATABASE_URL

# Down migration
db/migrate/down:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- down && \
	unset DATABASE_URL

# Generate a new migration file
db/migrate/generate:
	@if [ -z "$(NAME)" ]; then \
		echo "Error: MIGRATION_NAME is not set. Usage: make db/migrate/generate MIGRATION_NAME=<name>"; \
		exit 1; \
	fi
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- generate $(NAME) && \
	unset DATABASE_URL

# Apply all pending migrations
db/migrate/apply:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run && \
	unset DATABASE_URL

# Apply first N pending migrations
db/migrate/up-n:
	@if [ -z "$(NUM)" ]; then \
		echo "Warning: NUM is not set. Defaulting to 10 migrations."; \
		NUM=10; \
	fi
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- up -n $(NUM) && \
	unset DATABASE_URL

# Rollback last applied migration
db/migrate/rollback:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- down && \
	unset DATABASE_URL

# Rollback last N applied migrations
db/migrate/down-n:
	@if [ -z "$(NUM)" ]; then \
		echo "Warning: NUM is not set. Defaulting to 10 migrations."; \
		NUM=10; \
	fi
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- down -n $(NUM) && \
	unset DATABASE_URL

# Drop all tables, then reapply all migrations
db/migrate/fresh:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- fresh && \
	unset DATABASE_URL

# Rollback all applied migrations, then reapply all migrations
db/migrate/refresh:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- refresh && \
	unset DATABASE_URL

# Rollback all applied migrations
db/migrate/reset:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- reset && \
	unset DATABASE_URL

# Check the status of all migrations
db/migrate/status:
	@export DATABASE_URL=$(DATABASE) && \
	cd backend/migration && \
	cargo run -- status && \
	unset DATABASE_URL

app/back/up:
	@echo "Starting backend..."
	@cd backend && cargo run --bin app

app/front/install:
	@echo "Starting frontend..."
	@cd frontend && pnpm install

app/front/up:
	@echo "Starting frontend..."
	@cd frontend && pnpm dev
