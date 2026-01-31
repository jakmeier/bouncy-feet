build-dev-container: Dockerfile
	docker build --build-context parent=../ -t jakmeier/bouncy-feet:dev-api .

push-dev-container:
	docker push jakmeier/bouncy-feet:dev-api

build-prod-container: Dockerfile
	docker build --build-context parent=../ -t jakmeier/bouncy-feet:bouncy_backend .

push-prod-container:
	docker push jakmeier/bouncy-feet:bouncy_backend