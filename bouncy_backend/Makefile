# The CI workflow publishes immutable images to the Forgejo OCI registry.
# This target is only for local testing; override IMAGE to use another registry.
IMAGE ?= bouncy-feet-backend:local

build-container: Dockerfile
	docker build --build-context parent=../ -t $(IMAGE) .

push-container: build-container
	docker push $(IMAGE)
