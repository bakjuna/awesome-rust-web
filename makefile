.PHONY: help
run-backend:
	cargo run --path backend

run-frontend:
	cargo run --path frontend

watch:
	cargo watch -q -c -w src/ -x run

watch-tests:
	cargo watch -q -c -w . -x "test -- --nocapture"

lint:
	cargo clippy --fix --allow-dirty --allow-staged

dcu:
	docker-compose up -d

dcs:
	docker-compose stop

dcd:
	docker-compose down -v

env:
	cp -n .env.sample .env || true

fix-docker-issue:
	echo 'export COMPOSE_DOCKER_CLI_BUILD=0' >> ~/.zshrc
	echo 'export DOCKER_BUILDKIT=0' >> ~/.zshrc
	source ~/.zshrc

migrate:
	sqlx migrate add

migrate-run:
	sqlx migrate run