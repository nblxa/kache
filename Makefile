.PHONY: build

REPO ?= ghcr.io/nblxa/kache
TAG ?= 0.1.0
IMAGE ?= "$(REPO):$(TAG)"
PLATFORM ?= linux/arm64
DOCKERFILE ?= build.dockerfile
CONTEXT ?= .

# Default target to build the image
build_amd64:
	docker build \
		-t $(IMAGE) \
		--cache-from $(REPO) \
		--platform $(PLATFORM) \
		-f $(DOCKERFILE) \
		$(CONTEXT)

build_arm64:
	docker build \
		-t $(IMAGE) \
		--cache-from $(REPO) \
		--platform linux/arm64 \
		-f $(DOCKERFILE) \
		$(CONTEXT)

push_arm64:
	docker push $(IMAGE)

# Build and push the Helm chart
helm_chart:
	# Change helm chart version to the current image tag
	sed -i "s|^version: .*|version: $(TAG)|" charts/kache/Chart.yaml
	sed -i "s|^appVersion: .*|appVersion: $(TAG)|" charts/kache/Chart.yaml
	helm package charts/kache --destination charts/dist

helm_push:
	# Push the Helm chart to the repository
	helm push charts/dist/kache-$(TAG).tgz oci://ghcr.io/nblxa/kache/charts
