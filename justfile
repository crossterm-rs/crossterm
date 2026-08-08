# These recipes are shortcuts for local development. The GitHub Actions workflows remain the
# source of truth for required CI policy.

manifest := read("Cargo.toml")

raw_msrv := replace_regex(manifest, '(?ms)\A.*?^[ \t]*rust-version[ \t]*=[ \t]*"([^"]+)".*\z', '$1')

msrv_version := if raw_msrv == manifest { error("Cargo.toml must define rust-version") } else { raw_msrv }

# List the available local checks.
default:
    @just --list

# Run the required checks for the current operating system.
ci: format clippy docs doctest test msrv features package dependency-policy workflow-integrity

# Check Rust formatting.
format:
    cargo fmt --all -- --check

# Check stable Rust warnings.
clippy:
    cargo clippy --locked --all-targets --all-features -- -D warnings

# Check generated documentation and links.
[env("RUSTDOCFLAGS", "-D warnings")]
docs:
    cargo doc --locked --all-features --no-deps

# Run documentation tests.
doctest:
    cargo test --locked --doc --all-features

# Run the main test suite.
test:
    cargo test --locked --all-targets --all-features -- --test-threads 1

# Check both configurations covered by the minimum supported Rust version.
msrv:
    rustup run {{ msrv_version }} cargo check --locked --lib --no-default-features
    rustup run {{ msrv_version }} cargo check --locked --lib --all-features

# Test the deliberate Unix feature compatibility points.
[unix]
features:
    cargo test --locked --lib -- --test-threads 1
    cargo test --locked --lib --features serde -- --test-threads 1
    cargo test --locked --lib --features event-stream,events -- --test-threads 1
    cargo test --locked --lib --no-default-features -- --test-threads 1
    cargo test --locked --lib --no-default-features --features events -- --test-threads 1
    cargo test --locked --lib --no-default-features \
        --features events,event-stream,use-dev-tty,bracketed-paste \
        -- --test-threads 1

# Test the supported Windows feature compatibility points.
[windows]
features:
    cargo test --locked --lib --no-default-features -- --test-threads 1
    cargo test --locked --lib --no-default-features --features events -- --test-threads 1

# Verify package contents while allowing the local changes under review. CI checks VCS cleanliness.
package:
    cargo package --locked --allow-dirty

# Check dependency advisories, licenses, and sources.
dependency-policy:
    cargo deny --locked check advisories licenses sources

# Check GitHub Actions syntax and security.
workflow-integrity:
    actionlint
    zizmor --offline --strict-collection .

# Preview Clippy lints expected in the next stable Rust release.
beta-clippy:
    rustup run beta cargo clippy --locked --all-targets --all-features -- -D warnings
