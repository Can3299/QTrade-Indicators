# QTrade Indicators

[![crates.io](https://img.shields.io/crates/v/qtrade-indicators.svg)](https://crates.io/crates/qtrade-indicators)
[![docs.rs](https://img.shields.io/docsrs/qtrade-indicators)](https://docs.rs/qtrade-indicators)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A collection of technical analysis indicators for trading, written in Rust. Each indicator is gated behind a Cargo feature flag so you only compile what you need.

## Provided Indicators

| Indicator | Feature | Description |
|-----------|---------|-------------|
| Median Price | `median_price` | `(high + low) / 2` |
| True Range | `tr` | Volatility measure: `max(H-L, \|H-pC\|, \|L-pC\|)` |
| Simple Moving Average | `sma` | Arithmetic mean over a sliding window |
| Smoothed Moving Average | `smma` | Modified moving average with smoother response |
| Exponential Moving Average | `ema` | Exponentially weighted moving average |
| Running Moving Average | `rma` | Wilder's smoothing (`α = 1/N`) |
| Weighted Moving Average | `wma` | Linearly weighted moving average |
| Average True Range | `atr` | ATR with 5 smoothing engines (SMA, SMMA, EMA, RMA, WMA) |
| MACD | `macd` | Moving Average Convergence Divergence (histogram, signal, MACD line) |
| Supertrend | `supertrend` | Trend-following indicator with upper/lower bands |
| Williams Fractals | `wf` | Local price extreme points for support/resistance |

## Feature Flags

This crate uses Cargo [feature flags](https://doc.rust-lang.org/cargo/reference/features.html) to enable indicators individually:

```toml
[dependencies]
qtrade-indicators = { version = "0.1", features = ["sma", "ema", "macd"] }
```

Some features automatically enable others:

- `atr` enables `tr`, `sma`, `smma`, `ema`, `rma`, `wma`
- `macd` enables `ema`

For development and testing, enable all features:

```toml
[dependencies]
qtrade-indicators = { version = "0.1", features = ["dev"] }
```

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
qtrade-indicators = { version = "0.1", features = ["sma"] }
```

### Simple Moving Average

```rust
use qtrade_indicators::simple_moving_average::{SettingSma, calculate_sma};

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let setting = SettingSma { period: 3 };
let sma = calculate_sma(&data, &setting).unwrap();
// sma = [0.0, 0.0, 2.0, 3.0, 4.0]
```

### MACD

```rust
use qtrade_indicators::moving_average_convergence_divergence::{
    SettingMacd, calculate_macd,
};

let data: Vec<f64> = (10..31).map(|i| i as f64).collect();
let setting = SettingMacd {
    fast_length: 3,
    slow_length: 5,
    signal_smooth: 3,
};
let macd = calculate_macd(&data, &setting).unwrap();
```

### Supertrend

```rust
use qtrade_indicators::supertrend::{SettingSupertrend, calculate_supertrend};

let close = vec![10.0, 11.0, 12.0, 9.0, 8.0, 13.0];
let median = vec![10.0, 10.5, 11.5, 9.5, 8.5, 12.5];
let atr = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
let setting = SettingSupertrend { factor: 2.0 };

let st = calculate_supertrend(&close, &median, &atr, &setting).unwrap();
println!("Trend: {:?}", st[0].trend); // Up
```

## API Documentation

Full API documentation is available on [docs.rs](https://docs.rs/qtrade-indicators).

## Minimum Supported Rust Version (MSRV)

Rust **1.85** or later (Rust 2024 edition).

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
