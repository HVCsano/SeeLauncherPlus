default:
    just dev

[windows]
set shell := ["powershell"]

dev:
    cargo watch -x run

update-all: update-tools
    cargo upgrade

update-tools: update-rust install-crates

update-rust:
    rustup update

install-crates:
    cargo install cargo-binstall
    cargo binstall cargo-edit just-lsp cargo-watch