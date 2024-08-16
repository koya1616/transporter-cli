build:
	docker compose -f compose.yml build

up:
	docker compose -f compose.yml up -d

exec:
	docker exec -it cli /bin/bash
