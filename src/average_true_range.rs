//! Average True Range (ATR) — volatility indicator with 5 smoothing engines.
/*!
Measures market volatility by smoothing the True Range with a choice of:
SMA, SMMA, EMA, RMA, or WMA.

Gated behind `#[cfg(feature = "atr")]`. Automatically enables `tr`, `sma`, `smma`,
`ema`, `rma`, and `wma`.

# Algorithm

```text
1. Compute True Range: tr = calculate_tr(close, high, low)
2. Smooth TR using the selected engine
```

# Example

```rust
use qtrade_indicators::average_true_range::{SettingAtr, SmoothEngine, calculate_atr};

let close = vec![10.0, 12.0, 11.0, 13.0, 14.0, 13.0];
let high  = vec![11.0, 13.0, 12.0, 14.0, 15.0, 14.0];
let low   = vec![9.0, 11.0, 10.0, 12.0, 13.0, 12.0];

let setting = SettingAtr { period: 3, smooth_engine: SmoothEngine::SMA };
let atr = calculate_atr(&close, &high, &low, &setting).unwrap();
// SMA(3) of TR: [0, 0, 2.333, 2.667, 2.333, 2.333]
```
*/

use crate::{
    exponential_moving_average::{SettingEma, calculate_ema},
    indicator_error::IndicatorError,
    running_moving_average::{SettingRma, calculate_rma},
    simple_moving_average::{SettingSma, calculate_sma},
    smoothed_moving_average::{SettingSmma, calculate_smma},
    true_range::calculate_tr,
    weighted_moving_average::{SettingWma, calculate_wma},
};
use tracing::instrument;

/// Smoothing engine selection for ATR calculation.
#[derive(Debug, Clone, Copy)]
pub enum SmoothEngine {
    /// Simple Moving Average
    SMA,
    /// Smoothed Moving Average
    SMMA,
    /// Exponential Moving Average
    EMA,
    /// Running Moving Average (Wilder's)
    RMA,
    /// Weighted Moving Average
    WMA,
}

/// Configuration for Average True Range calculation.
pub struct SettingAtr {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
    /// Which moving average variant to use for smoothing True Range values.
    pub smooth_engine: SmoothEngine,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_atr(
    candle_close: &[f64],
    candle_high: &[f64],
    candle_low: &[f64],
    setting: &SettingAtr,
) -> Result<Vec<f64>, IndicatorError> {
    if candle_close.is_empty() || candle_high.is_empty() || candle_low.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if candle_close.len() != candle_high.len() || candle_high.len() != candle_low.len() {
        return Err(IndicatorError::DifferentDataLength);
    }
    if setting.period == 0 {
        return Err(IndicatorError::ImproperSetting);
    }
    if candle_close.len() <= setting.period {
        return Err(IndicatorError::ImproperDataLength);
    }

    let true_range: Vec<f64> = calculate_tr(candle_close, candle_high, candle_low)?;

    let average_true_range: Vec<f64> = match setting.smooth_engine {
        SmoothEngine::SMA => calculate_sma(
            &true_range,
            &SettingSma {
                period: setting.period,
            },
        )?,
        SmoothEngine::SMMA => calculate_smma(
            &true_range,
            &SettingSmma {
                period: setting.period,
            },
        )?,
        SmoothEngine::EMA => calculate_ema(
            &true_range,
            &SettingEma {
                period: setting.period,
            },
        )?,
        SmoothEngine::RMA => calculate_rma(
            &true_range,
            &SettingRma {
                period: setting.period,
            },
        )?,
        SmoothEngine::WMA => calculate_wma(
            &true_range,
            &SettingWma {
                period: setting.period,
            },
        )?,
    };

    Ok(average_true_range)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_data() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = vec![10.0, 12.0, 11.0, 13.0, 14.0, 13.0];
        let high = vec![11.0, 13.0, 12.0, 14.0, 15.0, 14.0];
        let low = vec![9.0, 11.0, 10.0, 12.0, 13.0, 12.0];
        (close, high, low)
    }

    #[test]
    fn test_calculate_atr_sma_engine() {
        let (close, high, low) = make_test_data();
        let setting = SettingAtr {
            period: 3,
            smooth_engine: SmoothEngine::SMA,
        };
        let result = calculate_atr(&close, &high, &low, &setting).unwrap();
        assert_eq!(result.len(), 6);
        // TR values: [2.0, max(2,3,1)=3, max(2,0,2)=2, max(2,3,1)=3, max(2,2,0)=2, max(2,0,2)=2]
        // With SMA period=3: [0,0, (2+3+2)/3=2.333, (3+2+3)/3=2.667, (2+3+2)/3=2.333, (3+2+2)/3=2.333]
        assert!((result[2] - 7.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_atr_all_engines_same_length() {
        let (close, high, low) = make_test_data();
        let engines = [
            SmoothEngine::SMA,
            SmoothEngine::SMMA,
            SmoothEngine::EMA,
            SmoothEngine::RMA,
            SmoothEngine::WMA,
        ];
        for engine in engines {
            let setting = SettingAtr {
                period: 3,
                smooth_engine: engine,
            };
            let result = calculate_atr(&close, &high, &low, &setting).unwrap();
            assert_eq!(
                result.len(),
                6,
                "ATR with {:?} should return 6 values",
                engine
            );
        }
    }

    #[test]
    fn test_calculate_atr_different_engines_different_values() {
        let (close, high, low) = make_test_data();
        let sma = calculate_atr(
            &close,
            &high,
            &low,
            &SettingAtr {
                period: 3,
                smooth_engine: SmoothEngine::SMA,
            },
        )
        .unwrap();
        let ema = calculate_atr(
            &close,
            &high,
            &low,
            &SettingAtr {
                period: 3,
                smooth_engine: SmoothEngine::EMA,
            },
        )
        .unwrap();
        // At least one value differs between SMA and EMA
        let differs = sma
            .iter()
            .zip(ema.iter())
            .any(|(a, b)| (a - b).abs() > 1e-10);
        assert!(differs, "SMA and EMA should produce different ATR values");
    }

    #[test]
    fn test_calculate_atr_empty() {
        assert!(matches!(
            calculate_atr(
                &[],
                &[1.0],
                &[1.0],
                &SettingAtr {
                    period: 3,
                    smooth_engine: SmoothEngine::SMA
                }
            )
            .unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_atr_length_mismatch() {
        assert!(matches!(
            calculate_atr(
                &[1.0, 2.0],
                &[1.0],
                &[1.0],
                &SettingAtr {
                    period: 3,
                    smooth_engine: SmoothEngine::SMA
                }
            )
            .unwrap_err(),
            IndicatorError::DifferentDataLength
        ));
    }

    #[test]
    fn test_calculate_atr_zero_period() {
        assert!(matches!(
            calculate_atr(
                &[1.0, 2.0, 3.0],
                &[1.0, 2.0, 3.0],
                &[1.0, 2.0, 3.0],
                &SettingAtr {
                    period: 0,
                    smooth_engine: SmoothEngine::SMA
                }
            )
            .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }
}
