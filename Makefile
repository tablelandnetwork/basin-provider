HEAD_SHORT ?= $(shell git rev-parse --short HEAD)

lint:
	cargo clippy --fix
.PHONY: lint

format:
	cargo fmt --all
	cargo tomlfmt
	(cd ./lib/evm; cargo tomlfmt)
	(cd ./lib/protocol; cargo tomlfmt)
	(cd ./lib/worker; cargo tomlfmt)
	(cd ./lib/status_checker; cargo tomlfmt)
.PHONY: format

install:
	cargo install --path lib/worker
.PHONY: install

test:
	cargo test
.PHONY: test

build-images:
	docker build --build-arg CRATE=basin_worker -t textile/basin_worker:${HEAD_SHORT} .
	docker tag textile/basin_worker:${HEAD_SHORT} textile/basin_worker:latest
.PHONY: build-images

push-images:
	docker image push textile/basin_worker:${HEAD_SHORT}
	docker image push textile/basin_worker:latest
.PHONY: push-images
