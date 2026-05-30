//! Technical analysis indicators for trading.
/*!
Each indicator is gated behind a Cargo feature flag so you only compile what you need.

# Feature Flags

| Feature | Module | Description |
|---------|--------|-------------|
| `median_price` | `median_price` | `(high + low) / 2` |
| `tr` | `true_range` | True Range volatility measure |
| `wf` | `williams_fractals` | Williams Fractals |
| `sma` | `simple_moving_average` | Simple Moving Average |
| `smma` | `smoothed_moving_average` | Smoothed Moving Average |
| `ema` | `exponential_moving_average` | Exponential Moving Average |
| `rma` | `running_moving_average` | Running Moving Average (Wilder's) |
| `wma` | `weighted_moving_average` | Weighted Moving Average |
| `atr` | `average_true_range` | Average True Range (auto-enables tr, sma, smma, ema, rma, wma) |
| `macd` | `moving_average_convergence_divergence` | MACD (auto-enables ema) |
| `supertrend` | `supertrend` | Supertrend indicator |

Use the `dev` feature to enable all indicators:
```toml
[dependencies]
qtrade-indicators = { version = "0.1", features = ["dev"] }
```

# Indicator Enum

The [`Indicator`] enum provides a named identifier for each available indicator.

```rust
use qtrade_indicators::Indicator;

let name = Indicator::AverageTrueRange.to_string();
assert_eq!(name, "Average True Range");
```
*/

use std::fmt;
pub mod indicator_error;

pub enum Indicator {
    MedianPrice,                        // median_price
    TrueRange,                          // tr
    WilliamsFractals,                   // wf
    SimpleMovingAverage,                // sma
    SmoothedMovingAverage,              // smma
    ExponentialMovingAverage,           // ema
    RunningMovingAverage,               // rma
    WeightedMovingAverage,              // wma
    AverageTrueRange,                   // atr
    MovingAverageConvergenceDivergence, // macd
    Supertrend,                         // supertrend
}
impl Indicator {
    fn as_str(&self) -> &'static str {
        match self {
            Indicator::MedianPrice => "Median Price",
            Indicator::TrueRange => "True Range",
            Indicator::WilliamsFractals => "Williams Fractals",
            Indicator::SimpleMovingAverage => "Simple Moving Average",
            Indicator::SmoothedMovingAverage => "Smoothed Moving Average",
            Indicator::ExponentialMovingAverage => "Exponential Moving Average",
            Indicator::RunningMovingAverage => "Running Moving Average",
            Indicator::WeightedMovingAverage => "Weighted Moving Average",
            Indicator::AverageTrueRange => "Average True Range",
            Indicator::MovingAverageConvergenceDivergence => {
                "Moving Average Convergence Divergence"
            }
            Indicator::Supertrend => "Supertrend",
        }
    }
}

impl fmt::Display for Indicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "median_price")]
pub mod median_price;

#[cfg(feature = "tr")]
pub mod true_range;

#[cfg(feature = "wf")]
pub mod williams_fractals;

#[cfg(feature = "sma")]
pub mod simple_moving_average;

#[cfg(feature = "smma")]
pub mod smoothed_moving_average;

#[cfg(feature = "ema")]
pub mod exponential_moving_average;

#[cfg(feature = "rma")]
pub mod running_moving_average;

#[cfg(feature = "wma")]
pub mod weighted_moving_average;

#[cfg(feature = "atr")]
pub mod average_true_range;

#[cfg(feature = "macd")]
pub mod moving_average_convergence_divergence;

#[cfg(feature = "supertrend")]
pub mod supertrend;
