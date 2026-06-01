# Contributing to qtrade-indicators

First off, thank you for considering contributing to qtrade-indicators. This project is a Rust library of technical analysis indicators, and every contribution — whether a bug report, a feature request, a documentation improvement, or a pull request — is appreciated.

This document outlines the workflow, conventions, and standards we follow. Please read it before opening an issue or pull request.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [CI Gate](#ci-gate)
- [Coding Conventions](#coding-conventions)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)

---

## Code of Conduct

This project adheres to the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you are expected to uphold this code. Please report unacceptable behaviour to the project maintainers.

---

## Getting Started

### Prerequisites

- **Rust toolchain**: nightly (benchmarks use `#![feature(test)]`, which requires nightly).
- **System dependencies**: a C linker (`gcc` or `clang`), typically already present.

### Setting Up

```bash
git clone https://github.com/Can3299/qtrade-indicators
cd qtrade-indicators
cargo build --all-features
```

---

## Development Workflow

1. **Fork and branch** — create a feature branch from `main`. Use a descriptive name:
   ```
   git checkout -b feat/add-wilder-swing-index
   git checkout -b fix/panic-in-supertrend
   git checkout -b docs/update-readme
   ```

2. **Make changes** — follow the conventions below.

3. **Run the CI gate locally** (see [CI Gate](#ci-gate)).

4. **Commit** — write a concise, descriptive commit message.

5. **Push and open a pull request** against `main`.

---

## CI Gate

Before committing or opening a pull request, run these checks:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo bench --no-run --all-features
```

> `cargo clippy` is run with `-D warnings`, meaning any clippy lint is treated as a hard error. Address all warnings before pushing.

---

## Coding Conventions

### Language & Toolchain

- **Edition 2024**, nightly channel.
- Run `cargo fmt` before every commit (CI enforces `cargo fmt --check`).
- All warnings are denied in CI. Keep clippy happy.

### Error Handling

A single shared error enum (`IndicatorError`) is returned by every `calculate_*` function:

```rust
#[derive(Debug)]
pub enum IndicatorError {
    EmptyData,
    DifferentDataLength,
    ImproperDataLength,
    ImproperSetting,
}

impl fmt::Display for IndicatorError { /* ... */ }
impl std::error::Error for IndicatorError {}
```

Error variants are mapped explicitly in match arms — do not use `.map_err()` generically.

### Instrumentation

Every public `calculate_*` function is annotated with `#[tracing::instrument]`:

```rust
#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_sma(candle_data: &[f64], setting: &SettingSma) -> Result<Vec<f64>, IndicatorError> { ... }
```

This generates automatic tracing spans. Use `skip_all` to avoid capturing large input slices.

### Feature Gates

Each indicator is gated behind a Cargo feature flag in `Cargo.toml`:

```toml
[features]
default = []
sma = []
ema = []
atr = ["tr", "sma", "smma", "ema", "rma", "wma"]
macd = ["ema"]
dev = ["median_price", "tr", "wf", "sma", "smma", "ema", "rma", "wma", "atr", "macd", "supertrend"]
```

Use `#[cfg(feature = "...")]` to conditionally compile modules and tests.

### Configuration Structs

Each indicator defines a dedicated configuration struct:

```rust
pub struct SettingSma {
    pub period: usize,
}
```

### Validation-First Pattern

Every `calculate_*` function validates inputs upfront in a consistent order:

1. Empty data → `IndicatorError::EmptyData`
2. Invalid setting → `IndicatorError::ImproperSetting`
3. Insufficient data length → `IndicatorError::ImproperDataLength`
4. Length mismatch (multi-input indicators) → `IndicatorError::DifferentDataLength`

---

## Testing Guidelines

### Running Tests

```bash
cargo test --all-features              # full suite
cargo test --features sma,ema,macd     # selected indicators only
```

### Test Placement

- **Unit tests**: in a `#[cfg(test)] mod tests { ... }` block at the bottom of each source file.
- **Integration tests**: currently none. If you add some, place them in `tests/` at the crate root.

### Benchmarks

Benchmarks live in `benches/*.rs` at the crate root and use nightly-only `#![feature(test)]`:

```bash
cargo bench --no-run --all-features   # compile-check only
cargo bench --all-features            # run locally
```

---

## Documentation

- **Inline docs**: use `///` doc comments on all public items. Include at least a one-sentence description.
- **Module-level docs**: each source file starts with a `//!` outer doc comment and a `/*!` block containing algorithm pseudocode, a usage example, and the feature gate note.
- **README.md**: update if you change the build process, add dependencies, or alter the API.
- **CHANGELOG.md**: add an entry under `[Unreleased]` for any user-facing change (new feature, bug fix, breaking change). Follow [Keep a Changelog](https://keepachangelog.com/) conventions.

### Commit Messages

write a concise, descriptive commit message.

---

## Pull Request Process

1. Ensure your branch is up to date with the target branch (`main`).
2. Run the full [CI gate](#ci-gate) locally and confirm all checks pass.
3. Open a pull request with a clear title and description. Reference any related issues.
4. All CI checks must pass before merging.
5. Maintainers will review your changes. Expect constructive feedback; it is not personal.
6. Once approved, a maintainer will merge your PR. You may be asked to squash commits if the history is noisy.

### PR Checklist

Before opening a PR, ask yourself:

- [ ] Have I read the CONTRIBUTING.md file?
- [ ] Does the code compile without warnings?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` pass?
- [ ] Does `cargo fmt --check` pass?
- [ ] Do all existing tests pass (`cargo test --all-features`)?
- [ ] Have I added tests for new functionality?
- [ ] Have I updated or added documentation (doc comments, README.md)?
- [ ] Have I checked for look-ahead bias?
- [ ] Have I updated `CHANGELOG.md` if the change is user-facing?

---

## Issue Reporting

- **Bug reports**: include the full error output, the Rust version (`rustc --version`), and steps to reproduce.
- **Feature requests**: describe the problem you want to solve, not just the solution you have in mind. This helps the community explore better approaches.
- **Questions**: search existing issues and discussions before opening a new one.

---

## License

By contributing, you agree that your contributions will be licensed under the [Apache License, Version 2.0](LICENSE) — the same license as the project.
