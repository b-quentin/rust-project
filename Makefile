.PHONY: up dev

docker/up:
	@echo "Starting Docker containers..."
	@cd docker && docker compose up -d

back/up:
	@echo "Starting backend..."
	@cd backend && cargo run --bin app

front/up:
	@echo "Starting frontend..."
	@cd frontend && pnpm dev
