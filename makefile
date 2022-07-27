watch:
	docker-compose -f docker-compose.yaml up -d	&& docker-compose -f docker-compose.yaml exec actix_api_test bash