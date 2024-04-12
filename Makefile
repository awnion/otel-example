.DEFAULT_GOAL := help

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
		print "  " $$1 \
	}' $(MAKEFILE_LIST) | column -t -s ':'

run-services: ## Run docker services
	cd otel && docker compose up -d --remove-orphans

run-debug: run-services ## Run in debug
	cargo run

run: run-services ## Build the project
	cargo run -r

.PHONY: run-services run-debug run
