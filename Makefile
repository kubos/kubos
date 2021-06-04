.PHONY: docker-image test-release

GITTAG:=$(shell git describe --tags)
DOCKER_IMAGE=kubos/kubos-dev
DOCKER_TAG=v2.0.0-pre
DOCKER_CI_NAME=kubos-test

CI_TEST_COMMAND="/bin/bash -c cd && git clone https://github.com/kubos/kubos && cd kubos && pushd . && cd apis/app-api/python/ && pip3 install -r requirements.txt && pip3 install . && popd && cargo test --workspace --release"

docker-image:
	cd tools/dist/ && docker build -t $(DOCKER_IMAGE):$(GITTAG) .

test-release:
	docker run --name $(DOCKER_CI_NAME) --rm -t $(DOCKER_IMAGE):$(DOCKER_TAG) bash -c $(CI_TEST_COMMAND)