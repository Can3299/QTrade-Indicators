//! Smoothed Moving Average (SMMA) — a modified moving average with smoother response.
/*!
Also known as "Smoothed MA" or "Modified Moving Average". Places more weight on older
values than SMA, producing a smoother curve.

Gated behind `#[cfg(feature = "smma")]`.

# Algorithm

```text
SMMA[period-1] = sum(data[0..period]) / period        (initial SMA)
SMMA[i] = (SMMA[i-1] * (period - 1) + data[i]) / period   for i >= period
```

Equivalently: `SMMA[i] = SMMA[i-1] + (data[i] - SMMA[i-1]) / period`

# Example

```rust
use qtrade_indicators::smoothed_moving_average::{SettingSmma, calculate_smma};

let data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
let setting = SettingSmma { period: 3 };
let smma = calculate_smma(&data, &setting).unwrap();
// smma = [0.0, 0.0, 4.0, 5.333..., 6.888...]
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Configuration for Smoothed Moving Average calculation.
pub struct SettingSmma {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_smma(
    candle_data: &[f64],
    setting: &SettingSmma,
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

    let mut smma: Vec<f64> = Vec::with_capacity(candle_data.len());
    let mut prev_smma: f64;

    // First Period
    smma.extend(std::iter::repeat_n(0.0, setting.period - 1));

    // First SMMA
    prev_smma = candle_data.iter().take(setting.period).sum::<f64>() / setting.period as f64;
    smma.push(prev_smma);

    let period: f64 = setting.period as f64;
    let weight: f64 = period - 1.0;
    candle_data.iter().skip(setting.period).for_each(|&price| {
        prev_smma = (prev_smma * weight + price) / period;
        smma.push(prev_smma);
    });

    Ok(smma)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_smma_known_values() {
        let data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let setting = SettingSmma { period: 3 };
        let result = calculate_smma(&data, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        // First SMMA: (2+4+6)/3 = 4.0
        assert!((result[2] - 4.0).abs() < 1e-10);
        // Second SMMA: (4.0*2 + 8)/3 = 16/3 ≈ 5.333
        assert!((result[3] - 5.333_333_333_333).abs() < 1e-10);
        // Third SMMA: (5.333...*2 + 10)/3 = 20.666.../3 ≈ 6.888...
        assert!((result[4] - 6.888_888_888_888).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_smma_constant_data() {
        let data = vec![10.0; 10];
        let setting = SettingSmma { period: 4 };
        let result = calculate_smma(&data, &setting).unwrap();
        for &val in result.iter().skip(setting.period - 1) {
            assert!((val - 10.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_calculate_smma_empty() {
        assert!(matches!(
            calculate_smma(&[], &SettingSmma { period: 5 }).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_smma_zero_period() {
        assert!(matches!(
            calculate_smma(&[1.0], &SettingSmma { period: 0 }).unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_smma_short_data() {
        assert!(matches!(
            calculate_smma(&[1.0, 2.0], &SettingSmma { period: 5 }).unwrap_err(),
            IndicatorError::ImproperDataLength
        ));
    }

    #[test]
    fn test_calculate_smma_smoothes_more_than_sma() {
        let data = vec![100.0, 10.0, 100.0, 10.0, 100.0];
        let setting = SettingSmma { period: 3 };
        let result = calculate_smma(&data, &setting).unwrap();
        // SMMA should produce smoother (less extreme) values than simple average
        for &val in result.iter().skip(setting.period - 1) {
            assert!(val > 10.0 && val < 100.0);
        }
    }
}
