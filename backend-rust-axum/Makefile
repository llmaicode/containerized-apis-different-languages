.PHONY: build run test clean logs dev benchmark

# Image and container names
IMAGE_NAME := todo-axum
CONTAINER_NAME := todo-axum-container
PORT := 8080

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Run the container (requires PostgreSQL running separately)
run:
	docker run -d \
		--name $(CONTAINER_NAME) \
		-p $(PORT):8080 \
		-e DATABASE_URL=postgres://postgres:password@host.docker.internal:5432/todo_db \
		$(IMAGE_NAME)

# Run with docker-compose (includes PostgreSQL)
run-compose:
	docker-compose up -d --build

# Stop and remove container
stop:
	docker stop $(CONTAINER_NAME) || true
	docker rm $(CONTAINER_NAME) || true

# View logs
logs:
	docker logs -f $(CONTAINER_NAME)

# Test API endpoints
test:
	@echo "Testing API endpoints..."
	@sleep 3
	@curl -X POST http://localhost:$(PORT)/todos \
		-H "Content-Type: application/json" \
		-d '{"title": "Test Todo", "description": "Test description"}' \
		| jq .
	@curl http://localhost:$(PORT)/todos | jq .
	@curl http://localhost:$(PORT)/todos/1 | jq .

# Open Swagger UI
swagger:
	@echo "Opening Swagger UI at http://localhost:$(PORT)/docs"
	@xdg-open http://localhost:$(PORT)/docs 2>/dev/null || open http://localhost:$(PORT)/docs 2>/dev/null || echo "Open http://localhost:$(PORT)/docs in your browser"

# Clean up
clean: stop
	docker rmi $(IMAGE_NAME) || true

# Rebuild and run
rebuild: clean build run

# Show container status
status:
	docker ps -a --filter name=$(CONTAINER_NAME)

# Run locally with Cargo
dev:
	cargo run

# Check code
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy

# Run tests
test-unit:
	cargo test

# Run benchmark with k6
benchmark:
	@echo "Running k6 benchmark..."
	k6 run --out json=results/axum.json -e URL=http://localhost:$(PORT) ../benchmark/tests/todo_test.js

# Help
help:
	@echo "Available targets:"
	@echo "  build        - Build Docker image"
	@echo "  run          - Run container (needs external PostgreSQL)"
	@echo "  run-compose  - Run with docker-compose (includes PostgreSQL)"
	@echo "  stop         - Stop and remove container"
	@echo "  logs         - View container logs"
	@echo "  test         - Test API endpoints"
	@echo "  swagger      - Open Swagger UI"
	@echo "  clean        - Remove container and image"
	@echo "  rebuild      - Clean rebuild and run"
	@echo "  status       - Show container status"
	@echo "  dev          - Run locally with Cargo"
	@echo "  check        - Check code with cargo check"
	@echo "  fmt          - Format code with cargo fmt"
	@echo "  clippy       - Run clippy lints"
	@echo "  test-unit    - Run unit tests"
	@echo "  benchmark    - Run k6 benchmark"
	@echo "  help         - Show this help"