//! Moving Average Convergence Divergence (MACD) — trend-following momentum indicator.
/*!
Shows the relationship between two exponential moving averages of price.
Produces three series: MACD line, signal line, and histogram.

Gated behind `#[cfg(feature = "macd")]`. Automatically enables `ema`.

# Algorithm

```text
1. ema_fast = calculate_ema(data, SettingEma { period: fast_length })
2. ema_slow = calculate_ema(data, SettingEma { period: slow_length })
3. macd_line[i] = ema_fast[i] - ema_slow[i]
4. signal_line = calculate_ema(&macd_line, SettingEma { period: signal_smooth })
5. histogram[i] = macd_line[i] - signal_line[i]
```

# Example

```rust
use qtrade_indicators::moving_average_convergence_divergence::{SettingMacd, calculate_macd};

let data: Vec<f64> = (10..31).map(|i| i as f64).collect();
let setting = SettingMacd { fast_length: 3, slow_length: 5, signal_smooth: 3 };
let macd = calculate_macd(&data, &setting).unwrap();
assert!(macd[10].histogram == macd[10].macd_line - macd[10].signal_line);
```
*/

use crate::exponential_moving_average::{SettingEma, calculate_ema};
use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// A single MACD data point containing the MACD line, signal line, and histogram.
#[derive(Debug, Clone)]
pub struct MacdData {
    /// MACD line: fast EMA minus slow EMA.
    pub macd_line: f64,
    /// Signal line: EMA of the MACD line.
    pub signal_line: f64,
    /// Histogram: MACD line minus signal line.
    pub histogram: f64,
}

/// Configuration for MACD calculation.
pub struct SettingMacd {
    /// Fast EMA period (typically 12).
    pub fast_length: usize,
    /// Slow EMA period (typically 26).
    pub slow_length: usize,
    /// Signal line EMA period (typically 9).
    pub signal_smooth: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_macd(
    candle_data: &[f64],
    setting: &SettingMacd,
) -> Result<Vec<MacdData>, IndicatorError> {
    if candle_data.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if setting.fast_length == 0 {
        return Err(IndicatorError::ImproperSetting);
    }
    if setting.slow_length == 0 {
        return Err(IndicatorError::ImproperSetting);
    }
    if setting.signal_smooth == 0 {
        return Err(IndicatorError::ImproperSetting);
    }

    let ema_fast: Vec<f64> = calculate_ema(
        candle_data,
        &SettingEma {
            period: setting.fast_length,
        },
    )?;
    let ema_slow: Vec<f64> = calculate_ema(
        candle_data,
        &SettingEma {
            period: setting.slow_length,
        },
    )?;

    // MACD Line
    let macd_line: Vec<f64> = ema_fast
        .iter()
        .zip(&ema_slow)
        .map(|(fast, slow)| fast - slow)
        .collect();

    // Signal Line
    let signal_line: Vec<f64> = calculate_ema(
        &macd_line,
        &SettingEma {
            period: setting.signal_smooth,
        },
    )?;

    // Histogram and All
    let macd: Vec<MacdData> = macd_line
        .into_iter()
        .zip(signal_line)
        .map(|(macd_line, signal_line)| MacdData {
            macd_line,
            signal_line,
            histogram: macd_line - signal_line,
        })
        .collect();

    Ok(macd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_macd_known_values() {
        let data = vec![
            10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0,
            24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
        ];
        let setting = SettingMacd {
            fast_length: 3,
            slow_length: 5,
            signal_smooth: 3,
        };
        let result = calculate_macd(&data, &setting).unwrap();
        assert_eq!(result.len(), 21);
        // MACD line = EMA_fast - EMA_slow
        // The slow EMA (period=5) will have more leading zeros (4) than fast (2)
        // So first valid MACD will be at index 4 (the max of the two)
        for point in &result {
            assert!(point.macd_line.is_finite());
            assert!(point.signal_line.is_finite());
            assert!(point.histogram.is_finite());
        }
        // histogram = macd_line - signal_line
        assert!(
            (result[4].histogram - (result[4].macd_line - result[4].signal_line)).abs() < 1e-10
        );
        assert!(
            (result[10].histogram - (result[10].macd_line - result[10].signal_line)).abs() < 1e-10
        );
    }

    #[test]
    fn test_calculate_macd_constant_data() {
        let data = vec![50.0; 50];
        let setting = SettingMacd {
            fast_length: 12,
            slow_length: 26,
            signal_smooth: 9,
        };
        let result = calculate_macd(&data, &setting).unwrap();
        assert_eq!(result.len(), 50);
        // All values are finite
        for point in &result {
            assert!(point.macd_line.is_finite());
            assert!(point.signal_line.is_finite());
            assert!(point.histogram.is_finite());
        }
        // histogram must equal macd_line - signal_line for every point
        for (i, point) in result.iter().enumerate() {
            let expected_hist = point.macd_line - point.signal_line;
            assert!(
                (point.histogram - expected_hist).abs() < 1e-12,
                "Index {}: histogram {} != macd {} - signal {} = {}",
                i,
                point.histogram,
                point.macd_line,
                point.signal_line,
                expected_hist
            );
        }
    }

    #[test]
    fn test_calculate_macd_histogram_sign() {
        // Increasing data: fast EMA > slow EMA => positive MACD
        let data: Vec<f64> = (0..30).map(|i| 10.0 + i as f64).collect();
        let setting = SettingMacd {
            fast_length: 5,
            slow_length: 10,
            signal_smooth: 5,
        };
        let result = calculate_macd(&data, &setting).unwrap();
        for point in result.iter().skip(15) {
            assert!(
                point.macd_line > 0.0,
                "MACD line should be positive when price increases"
            );
        }
    }

    #[test]
    fn test_calculate_macd_empty() {
        assert!(matches!(
            calculate_macd(
                &[],
                &SettingMacd {
                    fast_length: 3,
                    slow_length: 5,
                    signal_smooth: 3
                }
            )
            .unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_macd_zero_fast_length() {
        assert!(matches!(
            calculate_macd(
                &[1.0],
                &SettingMacd {
                    fast_length: 0,
                    slow_length: 5,
                    signal_smooth: 3
                }
            )
            .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_macd_zero_slow_length() {
        assert!(matches!(
            calculate_macd(
                &[1.0],
                &SettingMacd {
                    fast_length: 3,
                    slow_length: 0,
                    signal_smooth: 3
                }
            )
            .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_macd_zero_signal_smooth() {
        assert!(matches!(
            calculate_macd(
                &[1.0],
                &SettingMacd {
                    fast_length: 3,
                    slow_length: 5,
                    signal_smooth: 0
                }
            )
            .unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }
}
