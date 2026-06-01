# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-05-30

### Added

- **Median Price** — `(high + low) / 2` midpoint calculation ([`median_price`] feature).
- **True Range** — volatility measure: max of high–low, high–prev close, low–prev close ([`tr`] feature).
- **Simple Moving Average (SMA)** — arithmetic mean over a sliding window ([`sma`] feature).
- **Smoothed Moving Average (SMMA)** — modified moving average with smoother response ([`smma`] feature).
- **Exponential Moving Average (EMA)** — exponentially weighted moving average ([`ema`] feature).
- **Running Moving Average (RMA)** — Wilder's smoothing with `alpha = 1 / N` ([`rma`] feature).
- **Weighted Moving Average (WMA)** — linearly weighted moving average ([`wma`] feature).
- **Average True Range (ATR)** — with five configurable smoothing engines (SMA, SMMA, EMA, RMA, WMA) ([`atr`] feature; auto-enables `tr`, `sma`, `smma`, `ema`, `rma`, `wma`).
- **Moving Average Convergence Divergence (MACD)** — MACD line, signal line, and histogram ([`macd`] feature; auto-enables `ema`).
- **Supertrend** — trend-following indicator with upper and lower bands ([`supertrend`] feature).
- **Williams Fractals** — local price extreme points for support and resistance detection ([`wf`] feature).
- **Feature-gated modules** — each indicator is a separate Cargo feature; consumers compile only what they need. A `dev` meta-feature enables all indicators.
- **Shared error type** — `IndicatorError` with variants `EmptyData`, `DifferentDataLength`, `ImproperDataLength`, and `ImproperSetting`.
- **Tracing instrumentation** — all public `calculate_*` functions annotated with `#[instrument]` spans.
- **Benchmarks** — nightly-only benchmarks under `benches/` for every indicator.
- **Apache 2.0 license** — project licensed under Apache 2.0.