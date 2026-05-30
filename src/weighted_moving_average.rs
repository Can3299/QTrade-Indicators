//! Weighted Moving Average (WMA) — linearly weighted moving average.
/*!
Assigns linearly increasing weights to values within the window, with the most recent
value receiving the highest weight.

Gated behind `#[cfg(feature = "wma")]`.

# Algorithm

```text
weight_sum = period * (period + 1) / 2

For each window data[i..i+period]:
  weighted_sum = sum(data[i+j] * (j + 1)) for j = 0..period
  WMA[i + period - 1] = weighted_sum / weight_sum
```

Weights are `1, 2, 3, ..., period` (linearly increasing, most recent gets highest weight).

# Example

```rust
use qtrade_indicators::weighted_moving_average::{SettingWma, calculate_wma};

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let setting = SettingWma { period: 3 };
let wma = calculate_wma(&data, &setting).unwrap();
// wma[2] = (1*1 + 2*2 + 3*3)/6 = 14/6 ≈ 2.333
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Configuration for Weighted Moving Average calculation.
pub struct SettingWma {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_wma(
    candle_data: &[f64],
    setting: &SettingWma,
) -> Result<Vec<f64>, IndicatorError> {
    if candle_data.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if setting.period == 0 {
        return Err(IndicatorError::ImproperSetting);
    }
    if candle_data.len() <= setting.period {
        return Err(IndicatorError::ImproperDataLength);
    }

    let mut wma: Vec<f64> = Vec::with_capacity(candle_data.len());
    let weight_sum: f64 = (setting.period as f64) * ((setting.period as f64) + 1.0) / 2.0;

    // First Period
    wma.extend(std::iter::repeat_n(0.0, setting.period - 1));

    for window in candle_data.windows(setting.period) {
        let weighted_sum: f64 = window
            .iter()
            .enumerate()
            .map(|(idx, price)| price * (idx + 1) as f64)
            .sum();

        wma.push(weighted_sum / weight_sum);
    }

    Ok(wma)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_wma_known_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let setting = SettingWma { period: 3 };
        let result = calculate_wma(&data, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        // weight_sum = 3*(3+1)/2 = 6
        // First window [1,2,3]: (1*1 + 2*2 + 3*3)/6 = (1+4+9)/6 = 14/6 ≈ 2.333
        assert!((result[2] - 14.0 / 6.0).abs() < 1e-10);
        // Second window [2,3,4]: (2*1 + 3*2 + 4*3)/6 = (2+6+12)/6 = 20/6 ≈ 3.333
        assert!((result[3] - 20.0 / 6.0).abs() < 1e-10);
        // Third window [3,4,5]: (3*1 + 4*2 + 5*3)/6 = (3+8+15)/6 = 26/6 ≈ 4.333
        assert!((result[4] - 26.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_wma_constant_data() {
        let data = vec![7.0; 6];
        let setting = SettingWma { period: 3 };
        let result = calculate_wma(&data, &setting).unwrap();
        for &val in result.iter().skip(setting.period - 1) {
            assert!((val - 7.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_calculate_wma_linear_increase() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let setting = SettingWma { period: 2 };
        let result = calculate_wma(&data, &setting).unwrap();
        // weight_sum = 2*3/2 = 3
        // WMA[2]: (2*1/3 + 3*2/3) ? no...
        // Window [1,2]: (1*1 + 2*2)/3 = 5/3 ≈ 1.667
        // Window [2,3]: (2*1 + 3*2)/3 = 8/3 ≈ 2.667
        // etc.
        assert!((result[1] - 5.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_wma_empty() {
        assert!(matches!(
            calculate_wma(&[], &SettingWma { period: 3 }).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_wma_zero_period() {
        assert!(matches!(
            calculate_wma(&[1.0], &SettingWma { period: 0 }).unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_wma_period_larger_than_data() {
        assert!(matches!(
            calculate_wma(&[1.0, 2.0], &SettingWma { period: 5 }).unwrap_err(),
            IndicatorError::ImproperDataLength
        ));
    }
}
