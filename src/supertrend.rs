//! Supertrend — trend-following indicator with upper/lower bands.
/*!
Built from median price, ATR, and a multiplier factor. Produces trend direction
(Up/Down) and upper/lower bands with continuity logic to smooth transitions.

Gated behind `#[cfg(feature = "supertrend")]`.

# Algorithm

```text
First candle:
  upper = median[0] + factor * atr[0]
  lower = median[0] - factor * atr[0]
  trend = Up if close[0] > lower else Down

For each subsequent candle i:
  raw_upper = median[i] + factor * atr[i]
  raw_lower = median[i] - factor * atr[i]

  upper = raw_upper if raw_upper < prev.upper OR close[i-1] > prev.upper else prev.upper
  lower = raw_lower if raw_lower > prev.lower OR close[i-1] < prev.lower else prev.lower

  if prev trend == Up:  trend = Down if close[i] <= lower else Up
  if prev trend == Down: trend = Up   if close[i] >= upper else Down
```

# Example

```rust
use qtrade_indicators::supertrend::{SettingSupertrend, calculate_supertrend};

let close  = vec![10.0, 11.0, 12.0, 9.0, 8.0, 13.0];
let median = vec![10.0, 10.5, 11.5, 9.5, 8.5, 12.5];
let atr    = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
let setting = SettingSupertrend { factor: 2.0 };

let st = calculate_supertrend(&close, &median, &atr, &setting).unwrap();
// Index 0: Up (close=10 > lower=8)
// Index 3: Down (close=9 <= lower=9.5) — trend flips
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Trend direction for a Supertrend data point.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrendType {
    /// Upward trend.
    Up,
    /// Downward trend.
    Down,
}

/// A single Supertrend data point containing the trend direction and band values.
#[derive(Debug)]
pub struct SupertrendData {
    /// Current trend direction.
    pub trend: TrendType,
    /// Upper band value (for visualization / short exit).
    pub upper_band: f64,
    /// Lower band value (for visualization / long exit).
    pub lower_band: f64,
}

/// Configuration for Supertrend calculation.
pub struct SettingSupertrend {
    /// ATR multiplier. Must be > 0.0 (e.g. 3.0).
    pub factor: f64,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_supertrend(
    candle_data: &[f64],
    median_price_data: &[f64],
    atr_data: &[f64],
    setting: &SettingSupertrend,
) -> Result<Vec<SupertrendData>, IndicatorError> {
    if candle_data.is_empty() || median_price_data.is_empty() || atr_data.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if candle_data.len() != median_price_data.len() || median_price_data.len() != atr_data.len() {
        return Err(IndicatorError::DifferentDataLength);
    }
    if setting.factor <= 0.0 {
        return Err(IndicatorError::ImproperSetting);
    }

    let mut supertrend: Vec<SupertrendData> = Vec::with_capacity(candle_data.len());

    // First Supertrend
    {
        let first_upper: f64 = median_price_data[0] + (setting.factor * atr_data[0]);
        let first_lower: f64 = median_price_data[0] - (setting.factor * atr_data[0]);

        supertrend.push(SupertrendData {
            trend: if candle_data[0] > first_lower {
                TrendType::Up
            } else {
                TrendType::Down
            },
            upper_band: first_upper,
            lower_band: first_lower,
        });
    }

    for i in 1..candle_data.len() {
        let prev: &SupertrendData = &supertrend[i - 1];
        let current_upper_calc: f64 = median_price_data[i] + (setting.factor * atr_data[i]);
        let current_lower_calc: f64 = median_price_data[i] - (setting.factor * atr_data[i]);

        let upper_band: f64 =
            if current_upper_calc < prev.upper_band || candle_data[i - 1] > prev.upper_band {
                current_upper_calc
            } else {
                prev.upper_band
            };

        let lower_band: f64 =
            if current_lower_calc > prev.lower_band || candle_data[i - 1] < prev.lower_band {
                current_lower_calc
            } else {
                prev.lower_band
            };

        let trend = match prev.trend {
            TrendType::Up => {
                if candle_data[i] <= lower_band {
                    TrendType::Down
                } else {
                    TrendType::Up
                }
            }
            TrendType::Down => {
                if candle_data[i] >= upper_band {
                    TrendType::Up
                } else {
                    TrendType::Down
                }
            }
        };

        supertrend.push(SupertrendData {
            trend,
            upper_band,
            lower_band,
        });
    }

    Ok(supertrend)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_custom_data() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = vec![10.0, 11.0, 12.0, 9.0, 8.0, 13.0];
        let median = vec![10.0, 10.5, 11.5, 9.5, 8.5, 12.5];
        let atr = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        (close, median, atr)
    }

    #[test]
    fn test_calculate_supertrend_basic() {
        let (close, median, atr) = make_custom_data();
        let setting = SettingSupertrend { factor: 2.0 };
        let result = calculate_supertrend(&close, &median, &atr, &setting).unwrap();
        assert_eq!(result.len(), 6);
        // First: upper = 10+2=12, lower = 10-2=8, close=10 > lower=8 => Up
        assert_eq!(result[0].trend, TrendType::Up);
        assert!((result[0].upper_band - 12.0).abs() < 1e-10);
        assert!((result[0].lower_band - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_supertrend_trend_flip_up_to_down() {
        let (close, median, atr) = make_custom_data();
        let setting = SettingSupertrend { factor: 2.0 };
        let result = calculate_supertrend(&close, &median, &atr, &setting).unwrap();
        // Index 3: close=9, median=9.5, lower_band=7.5 (continuity lower=8.5 from index 1)
        // prev lower at index 2 is 9.5 (calculated at i=2). Let me trace...
        // i=0: upper=12, lower=8
        // i=1: upper=12, lower=8.5 (calc=8.5, 8.5>8 => true => lower=8.5)
        // i=2: upper=12, lower=9.5 (calc=9.5, 9.5>8.5 => true => lower=9.5)
        // i=3: upper=11.5 (calc=11.5, 11.5<12 => true => upper=11.5), lower=9.5 (calc=7.5, 7.5>9.5=false, 12<9.5=false => lower=9.5)
        // prev trend=Up, close(9) > lower(9.5)? NO => flips to Down
        assert_eq!(result[3].trend, TrendType::Down);
        // Index 4: prev=Down, close(8) > upper(10.5)? NO => stays Down
        assert_eq!(result[4].trend, TrendType::Down);
    }

    #[test]
    fn test_calculate_supertrend_band_continuity() {
        let close = vec![10.0, 10.0, 10.0, 10.0];
        let median = vec![10.0, 10.0, 5.0, 5.0];
        let atr = vec![1.0, 1.0, 1.0, 1.0];
        let setting = SettingSupertrend { factor: 2.0 };
        let result = calculate_supertrend(&close, &median, &atr, &setting).unwrap();
        assert_eq!(result.len(), 4);
        // Index 0: upper=12, lower=8, close=10 > 8 => Up
        assert_eq!(result[0].trend, TrendType::Up);
        // Index 2: median=5, calc_upper=5+2=7, prev_upper=12
        // Condition: 7 < 12 (true) => upper_band = 7
        assert!((result[2].upper_band - 7.0).abs() < 1e-10);
        // Lower: calc_lower=5-2=3, prev_lower=8
        // Condition: 3 > 8 (false) || 10 < 8 (false) => keep prev_lower=8
        assert!((result[2].lower_band - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_supertrend_empty() {
        assert!(matches!(
            calculate_supertrend(&[], &[1.0], &[1.0], &SettingSupertrend { factor: 2.0 })
                .unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_supertrend_length_mismatch() {
        assert!(matches!(
            calculate_supertrend(
                &[1.0, 2.0],
                &[1.0],
                &[1.0],
                &SettingSupertrend { factor: 2.0 }
            )
            .unwrap_err(),
            IndicatorError::DifferentDataLength
        ));
    }

    #[test]
    fn test_calculate_supertrend_zero_factor() {
        assert!(matches!(
            calculate_supertrend(&[1.0], &[1.0], &[1.0], &SettingSupertrend { factor: 0.0 })
                .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_supertrend_negative_factor() {
        assert!(matches!(
            calculate_supertrend(&[1.0], &[1.0], &[1.0], &SettingSupertrend { factor: -1.0 })
                .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }
}
