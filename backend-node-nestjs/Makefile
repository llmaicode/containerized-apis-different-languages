.PHONY: build run test clean logs dev deps benchmark help

# Image and container names
IMAGE_NAME := todo-nestjs
CONTAINER_NAME := todo-nestjs-container
PORT := 3000

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Run the container (requires PostgreSQL running separately)
run:
	docker run -d \
		--name $(CONTAINER_NAME) \
		-p $(PORT):3000 \
		-e DB_HOST=host.docker.internal \
		-e DB_PORT=5432 \
		-e DB_USERNAME=postgres \
		-e DB_PASSWORD=password \
		-e DB_DATABASE=todo_db \
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

# Test the API endpoints
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

# Full rebuild and run
rebuild: clean build run

# Show container status
status:
	docker ps -a --filter name=$(CONTAINER_NAME)

# Run benchmark with k6
benchmark:
	@echo "Running k6 benchmark..."
	k6 run --out json=results/nestjs.json -e URL=http://localhost:$(PORT) ../benchmark/tests/todo_test.js

# Development: run locally with hot reload
dev:
	npm run start:dev

# Install dependencies
deps:
	npm install

# Build for production
prod-build:
	npm run build

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
	@echo "  benchmark    - Run k6 benchmark"
	@echo "  dev          - Run locally with hot reload"
	@echo "  deps         - Install npm dependencies"
	@echo "  prod-build   - Build for production"
	@echo "  help         - Show this help"