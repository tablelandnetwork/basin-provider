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

images:
	docker build --build-arg CRATE=basin_worker -t basin_worker:latest .
	docker build --build-arg CRATE=basin_exporter -t basin_exporter:latest .
.PHONY: images
