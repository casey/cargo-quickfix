all: build test clippy fmt-check forbid check-install

ci:
  RUSTFLAGS='--deny warnings' just all

pr: ci
  gh pr create --web

build:
  cargo build --all

test FILTER='':
  cargo test --all {{FILTER}}

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

forbid:
  ./bin/forbid

check-install:
  #!/usr/bin/env bash
  tmp=`mktemp -d`
  cargo install --path . --root $tmp
  $tmp/bin/quickfix --version

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
