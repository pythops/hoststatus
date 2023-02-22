default:
    @just --list

lint:
    @cargo clippy --workspace --all-features -- -D warnings
    @cargo fmt --all -- --check

build:
    @cargo build

update:
    @cargo upgrade
