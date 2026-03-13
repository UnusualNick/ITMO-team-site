.PHONY: run-frontend run-backend

run-frontend:
	trunk serve --open

run-backend:
	cd itmo-backend && cargo run
