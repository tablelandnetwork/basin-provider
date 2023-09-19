lint:
	cargo clippy --fix
.PHONY: lint

format:
	cargo fmt --all
	cargo tomlfmt
	(cd ./lib/evm; cargo tomlfmt)
	(cd ./lib/exporter; cargo tomlfmt)
	(cd ./lib/protocol; cargo tomlfmt)
	(cd ./lib/worker; cargo tomlfmt)
.PHONY: format

install:
	cargo install --path lib/worker
	cargo install --path lib/exporter
.PHONY: install

test:
	cargo test
.PHONY: test

build-images:
	docker build --build-arg CRATE=basin_worker -t textile/basin_worker:latest .
	docker build --build-arg CRATE=basin_exporter -t textile/basin_exporter:latest .
.PHONY: build-images

push-images:
	docker image push textile/basin_worker:latest
	docker image push textile/basin_exporter:latest
.PHONY: push-images
