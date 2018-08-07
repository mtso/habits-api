.PHONY: build

svc_name = $(shell basename $(shell pwd))

build:
	docker build -t mtso/$(svc_name):$(version) -f Dockerfile-build --build-arg SERVICE_NAME=$(svc_name) .
