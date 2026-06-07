.PHONY: run test db-up db-down migrate

db-up:
	docker compose up -d

db-down:
	docker compose down

migrate:
	sqlx migrate run

run: db-up migrate
	cargo run

test:
	cargo test
