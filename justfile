_list:
    @just --list

clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --all-features
    cargo hack --feature-powerset --depth=3 clippy --workspace

test:
    cargo test --all-features
    @just test-coverage-codecov
    @just test-coverage-lcov
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

test-coverage-codecov:
    cargo llvm-cov --workspace --all-features --codecov --output-path codecov.json

test-coverage-lcov:
    cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

doc:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

doc-watch:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features --open
    cargo watch -- RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

check:
    just --unstable --fmt --check
    npx -y prettier --check $(fd --hidden -e=md -e=yml)
    taplo lint
    cargo +nightly fmt -- --check

fmt:
    just --unstable --fmt
    nix fmt
    npx -y prettier --write $(fd --hidden -e=md -e=yml)
    taplo format
    cargo +nightly fmt
