# Contributing

I would appreciate any contributions to this crate. However, some things are handy to know.

## Code Style

### Import Order

All imports are semantically grouped and ordered. The order is:

- standard library (`use std::...`)
- external crates (`use rand::...`)
- current crate (`use crate::...`)
- parent module (`use super::..`)
- current module (`use self::...`)
- module declaration (`mod ...`)

There must be an empty line between groups. An example:

```rust
use crossterm_utils::{csi, write_cout, Result};

use crate::sys::{get_cursor_position, show_cursor};

use super::Cursor;
```

#### CLion Tips

The CLion IDE does this for you (_Menu_ -> _Code_ -> _Optimize Imports_). Be aware that the CLion sorts
imports in a group in a different way when compared to the `rustfmt`. It's effectively two steps operation
to get proper grouping & sorting:

* _Menu_ -> _Code_ -> _Optimize Imports_ - group & semantically order imports
* `cargo fmt` - fix ordering within the group

Second step can be automated via _CLion_ -> _Preferences_ ->
_Languages & Frameworks_ -> _Rust_ -> _Rustfmt_ -> _Run rustfmt on save_.  

### Max Line Length

| Type                 | Max line length |
|:---------------------|----------------:|
| Code                 |             100 |
| Comments in the code |             120 |
| Documentation        |             120 |

100 is the [`max_width`](https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#max_width)
default value.

120 is because of the GitHub. The editor & viewer width there is +- 123 characters. 

### Warnings

The code must be warning free. It's quite hard to find an error if the build logs are polluted with warnings.
If you decide to silent a warning with (`#[allow(...)]`), please add a comment why it's required.

Always consult the
[GitHub Actions](https://github.com/crossterm-rs/crossterm/actions/workflows/ci.yml) build logs.

### Forbidden Warnings

Search for `#![deny(...)]` in the code:

* `unused_must_use`
* `unused_imports`

## Local checks

The [`justfile`](../justfile) provides memorable shortcuts for the local checks. Install
[`just`](https://just.systems/man/en/packages.html), then run `just` to list the available recipes
or `just ci` to run the required checks for the current operating system. The GitHub Actions
workflows remain the source of truth for required CI policy.

Run the required checks before opening a pull request:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --all-features --no-deps
cargo test --locked --doc --all-features
cargo test --locked --all-targets --all-features -- --test-threads 1
cargo package --locked
cargo deny --locked check advisories licenses sources
actionlint
zizmor --offline --strict-collection .
```

Crossterm tests run single-threaded because some tests interact with process-wide terminal and
environment state.

The dependency policy check uses `cargo-deny`. Its reviewed license and source policies live in
[`deny.toml`](../deny.toml).

Install the additional CI tools using their maintained instructions:
[cargo-deny](https://embarkstudios.github.io/cargo-deny/getting-started/installation.html),
[Actionlint](https://github.com/rhysd/actionlint/blob/main/docs/install.md), and
[Zizmor](https://docs.zizmor.sh/installation/).

[Actionlint](https://github.com/rhysd/actionlint) understands the GitHub Actions workflow schema and
expression types, so it catches invalid workflow structure and expressions that a generic YAML
parser cannot. [Zizmor](https://docs.zizmor.sh/audits/) looks for security problems such as
excessive permissions, unpinned actions, persisted credentials, dangerous triggers, and template
injection. The required Zizmor check disables online audits so it remains safe and deterministic
for fork pull requests.

The minimum supported Rust version covers the library without default features and with all public
features:

```sh
rustup run 1.75.0 cargo check --locked --lib --no-default-features
rustup run 1.75.0 cargo check --locked --lib --all-features
```

On Unix, test the selected feature configurations:

```sh
cargo test --locked --lib -- --test-threads 1
cargo test --locked --lib --features serde -- --test-threads 1
cargo test --locked --lib --features event-stream,events -- --test-threads 1
cargo test --locked --lib --no-default-features -- --test-threads 1
cargo test --locked --lib --no-default-features --features events -- --test-threads 1
cargo test --locked --lib --no-default-features \
  --features events,event-stream,use-dev-tty,bracketed-paste -- --test-threads 1
```

On Windows, test the selected feature configurations:

```sh
cargo test --locked --lib --no-default-features -- --test-threads 1
cargo test --locked --lib --no-default-features --features events -- --test-threads 1
```

Pull requests run these checks on GitHub Actions. Beta and nightly test compatibility checks run
weekly and can also be started manually. Pull requests also run the advisory beta Clippy check
described below.

## CI design

Crossterm's CI follows these constraints:

- Required checks use stable Rust and cover formatting, warnings, documentation, tests, the
  supported Rust version, selected feature configurations, and the publishable package.
- Operating-system coverage is used where platform behavior matters. Selected feature tests run on
  a representative platform to keep pull-request feedback timely.
- The feature configurations are deliberate compatibility points rather than an exhaustive
  powerset. [`cargo-hack`](https://github.com/taiki-e/cargo-hack) may be useful later if broader
  coverage justifies the extra runtime, policy, and tool maintenance.
- Beta Clippy is advisory. It warns about lints likely to reach the next stable toolchain without
  making pull requests fail because an upstream beta toolchain changed.
- Dependency policy covers the complete feature graph. Vulnerable or yanked packages, unapproved
  licenses, and unreviewed registries or Git sources block a pull request.
- Actionlint checks workflow structure and expressions, while Zizmor audits workflow security.
- `ci-success` is the stable check intended for branch protection. Individual jobs remain visible
  for diagnosis, but repository rules do not need to track every matrix job name.
- Pull-request CI is read-only and has no release or publishing authority.

Run the advisory Clippy check locally with:

```sh
rustup run beta cargo clippy --locked --all-targets --all-features -- -D warnings
```
