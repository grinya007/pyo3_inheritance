.PHONY = install help test

BASE := $(shell /bin/pwd)
POETRY = $(shell which poetry)

install:  ## Install both Prod and Dev Packages in Poetry
	poetry install

clippy:  # Run cargo clippy
	cargo clippy --all-targets --all-features -- -D warnings

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

test:  ## Cargo Test ( only CBC )
	cargo test

pytest:
	$(POETRY) run maturin develop -r && \
	$(POETRY) run pytest tests -v -s

build_wheel:
	$(POETRY) run maturin build -f --strip --release --out "${BASE}/release"

run_pre_commit:  ## Run pre-commit on the project files
	$(POETRY) run pre-commit run --all-files

update_pre_commit:  ## Run pre-commit auto update
	$(POETRY) run pre-commit autoupdate

