.PHONY: build test check coverage coverage-html clean

build:
	cargo build

check:
	cargo check --all-targets

test:
	cargo test --all-targets

# Requires: cargo install cargo-llvm-cov
coverage:
	cargo llvm-cov --all-features --workspace --fail-under-lines 85

coverage-html:
	cargo llvm-cov --all-features --workspace --html --open

coverage-ci:
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info --fail-under-lines 85

clean:
	cargo clean

# Pipeline tooling
pipeline-install:
	cd tools/pipeline_runner && poetry install

pipeline-check:
	cd tools/pipeline_runner && poetry run pipeline check-all
