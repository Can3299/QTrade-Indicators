//! Simple Moving Average (SMA) — arithmetic mean over a sliding window.
/*!
Gated behind `#[cfg(feature = "sma")]`.

The first `period - 1` elements of the output are `0.0` (leading padding).
Uses a sliding window sum for O(N) performance.

# Algorithm

```text
SMA[i] = 0.0                              for i < period - 1
SMA[period-1] = sum(data[0..period]) / period
SMA[i] = SMA[i-1] + (data[i] - data[i-period]) / period   for i >= period
```

# Example

```rust
use qtrade_indicators::simple_moving_average::{SettingSma, calculate_sma};

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let setting = SettingSma { period: 3 };
let sma = calculate_sma(&data, &setting).unwrap();
// sma = [0.0, 0.0, 2.0, 3.0, 4.0]
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Configuration for Simple Moving Average calculation.
pub struct SettingSma {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_sma(
    candle_data: &[f64],
    setting: &SettingSma,
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

    let mut sma = Vec::with_capacity(candle_data.len());
    let mut current_window_sum: f64 = candle_data[..setting.period].iter().sum();

    // First Period
    sma.extend(std::iter::repeat_n(0.0, setting.period - 1));

    // First SMA
    sma.push(current_window_sum / setting.period as f64);

    for i in setting.period..candle_data.len() {
        current_window_sum += candle_data[i] - candle_data[i - setting.period];
        sma.push(current_window_sum / setting.period as f64);
    }

    Ok(sma)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sma_known_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let setting = SettingSma { period: 3 };
        let result = calculate_sma(&data, &setting).unwrap();
        assert_eq!(result.len(), 5);
        // First period-1 = 2 elements are 0.0
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        // First SMA: (1+2+3)/3 = 2.0
        assert!((result[2] - 2.0).abs() < 1e-10);
        // Second SMA: (2+3+4)/3 = 3.0
        assert!((result[3] - 3.0).abs() < 1e-10);
        // Third SMA: (3+4+5)/3 = 4.0
        assert!((result[4] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_sma_minimum_length() {
        // Data length must be STRICTLY greater than period (len > period)
        let data = vec![10.0, 20.0, 30.0, 40.0];
        let setting = SettingSma { period: 3 };
        let result = calculate_sma(&data, &setting).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        assert!((result[2] - 20.0).abs() < 1e-10);
        assert!((result[3] - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_sma_period_one() {
        let data = vec![5.0, 10.0, 15.0];
        let setting = SettingSma { period: 1 };
        let result = calculate_sma(&data, &setting).unwrap();
        assert_eq!(result.len(), 3);
        // period-1 = 0 leading zeros, first SMA = first element / 1 = 5.0
        assert!((result[0] - 5.0).abs() < 1e-10);
        assert!((result[1] - 10.0).abs() < 1e-10);
        assert!((result[2] - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_sma_constant_data() {
        let data = vec![42.0; 10];
        let setting = SettingSma { period: 4 };
        let result = calculate_sma(&data, &setting).unwrap();
        for &val in result.iter().skip(setting.period - 1) {
            assert!((val - 42.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_calculate_sma_empty_data() {
        let setting = SettingSma { period: 5 };
        assert!(matches!(
            calculate_sma(&[], &setting).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_sma_zero_period() {
        let setting = SettingSma { period: 0 };
        assert!(matches!(
            calculate_sma(&[1.0, 2.0], &setting).unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_sma_period_larger_than_data() {
        let setting = SettingSma { period: 10 };
        assert!(matches!(
            calculate_sma(&[1.0, 2.0, 3.0], &setting).unwrap_err(),
            IndicatorError::ImproperDataLength
        ));
    }

    #[test]
    fn test_calculate_sma_large_dataset() {
        let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let setting = SettingSma { period: 50 };
        let result = calculate_sma(&data, &setting).unwrap();
        assert_eq!(result.len(), 1000);
        // Check last value: average of 950..=999
        let last_sum: f64 = (950..1000).map(|i| i as f64).sum();
        let last_expected = last_sum / 50.0;
        assert!((result[999] - last_expected).abs() < 1e-10);
    }
}
