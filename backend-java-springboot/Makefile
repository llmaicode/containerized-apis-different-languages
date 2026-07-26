.PHONY: build run test clean logs package

# Image and container names
IMAGE_NAME := todo-spring
CONTAINER_NAME := todo-spring-container
PORT := 8080

# Build the Docker image
build:
	docker build -t $(IMAGE_NAME) .

# Run the container (requires PostgreSQL running separately)
run:
	docker run -d \
		--name $(CONTAINER_NAME) \
		-p $(PORT):8080 \
		-e SPRING_DATASOURCE_URL=jdbc:postgresql://host.docker.internal:5432/todo_db \
		-e SPRING_DATASOURCE_USERNAME=postgres \
		-e SPRING_DATASOURCE_PASSWORD=password \
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
	@sleep 5
	@curl -X POST http://localhost:$(PORT)/todos \
		-H "Content-Type: application/json" \
		-d '{"title": "Test Todo", "description": "Test description"}' \
		| jq .
	@curl http://localhost:$(PORT)/todos | jq .
	@curl http://localhost:$(PORT)/todos/1 | jq .

# Open Swagger UI
swagger:
	@echo "Opening Swagger UI at http://localhost:$(PORT)/swagger-ui/index.html"
	@xdg-open http://localhost:$(PORT)/swagger-ui/index.html 2>/dev/null || open http://localhost:$(PORT)/swagger-ui/index.html 2>/dev/null || echo "Open http://localhost:$(PORT)/swagger-ui/index.html in your browser"

# Clean up
clean: stop
	docker rmi $(IMAGE_NAME) || true

# Rebuild and run
rebuild: clean build run

# Show container status
status:
	docker ps -a --filter name=$(CONTAINER_NAME)

# Run locally with Maven
dev:
	./mvnw spring-boot:run

# Package JAR
package:
	./mvnw clean package -DskipTests

# Run tests
test-unit:
	./mvnw test

# Run benchmark with k6
benchmark:
	@echo "Running k6 benchmark..."
	k6 run --out json=results/spring.json -e URL=http://localhost:$(PORT) ../benchmark/tests/todo_test.js

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
	@echo "  dev          - Run locally with Maven"
	@echo "  package      - Package JAR file"
	@echo "  test-unit    - Run unit tests"
	@echo "  benchmark    - Run k6 benchmark"
	@echo "  help         - Show this help"