# Contributing

Thanks for contributing to this TinyChain-managed dependency.

## Local validation

From a standalone checkout, run `bash .github/scripts/bootstrap-path-deps.sh`
before Cargo commands. It fetches missing sibling path dependencies, including
the JSON codec used by allocation regression tests and its transitive dependencies.
Existing sibling checkouts are preserved. CI uses the same setup.

Run `cargo fmt --all -- --check`, `cargo check`, and `cargo test` to match CI.
Run `cargo clippy --all-targets --all-features -- -D warnings` for strict linting.

## Licensing and IP

By contributing code, you represent that (a) you authored the work (or otherwise
have the rights to contribute it), (b) the contribution is unencumbered by
third-party intellectual property claims, and (c) you transfer and assign all
right, title, and interest in the contribution to The TinyChain Contributors for
distribution under the Apache 2.0 license (see `LICENSE`).
