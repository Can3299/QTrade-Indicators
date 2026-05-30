//! Running Moving Average (RMA) — Wilder's smoothing method.
/*!
Also known as Modified Exponential Average. Uses `alpha = 1 / period`, making it
smoother than EMA for the same period. This is the smoothing method used in
Welles Wilder's ATR.

Gated behind `#[cfg(feature = "rma")]`.

# Algorithm

```text
alpha = 1 / period

RMA[i] = 0.0                                      for i < period - 1
RMA[period-1] = SMA(data[0..period])               (seed with simple average)
RMA[i] = alpha * data[i] + (1 - alpha) * RMA[i-1]  for i >= period
```

The only difference from EMA is the alpha formula: `1/N` instead of `2/(N+1)`,
giving RMA a longer memory (slower response to new values).

# Example

```rust
use qtrade_indicators::running_moving_average::{SettingRma, calculate_rma};

let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
let setting = SettingRma { period: 4 };
let rma = calculate_rma(&data, &setting).unwrap();
// rma = [0.0, 0.0, 0.0, 25.0, 31.25]  (alpha = 0.25)
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Configuration for Running Moving Average (Wilder's) calculation.
pub struct SettingRma {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_rma(
    candle_data: &[f64],
    setting: &SettingRma,
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

    let mut rma: Vec<f64> = Vec::with_capacity(candle_data.len());
    let alpha: f64 = 1.0 / setting.period as f64;
    let inv_alpha = 1.0 - alpha;

    // First Period
    rma.extend(std::iter::repeat_n(0.0, setting.period - 1));

    let mut last_rma: f64 =
        candle_data.iter().take(setting.period).sum::<f64>() / setting.period as f64;
    rma.push(last_rma);

    for &price in candle_data.iter().skip(setting.period) {
        let current_rma = alpha.mul_add(price, inv_alpha * last_rma);

        last_rma = current_rma;
        rma.push(current_rma);
    }

    Ok(rma)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rma_known_values() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let setting = SettingRma { period: 4 };
        let result = calculate_rma(&data, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 0.0);
        // First RMA: (10+20+30+40)/4 = 25.0
        assert!((result[3] - 25.0).abs() < 1e-10);
        // alpha = 1/4 = 0.25
        // RMA_5 = 0.25*50 + 0.75*25 = 12.5 + 18.75 = 31.25
        assert!((result[4] - 31.25).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_rma_constant_data() {
        let data = vec![30.0; 8];
        let setting = SettingRma { period: 3 };
        let result = calculate_rma(&data, &setting).unwrap();
        for &val in result.iter().skip(setting.period - 1) {
            assert!((val - 30.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_calculate_rma_empty() {
        assert!(matches!(
            calculate_rma(&[], &SettingRma { period: 5 }).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_rma_zero_period() {
        assert!(matches!(
            calculate_rma(&[1.0], &SettingRma { period: 0 }).unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }

    #[test]
    fn test_calculate_rma_short_data() {
        assert!(matches!(
            calculate_rma(&[1.0, 2.0], &SettingRma { period: 5 }).unwrap_err(),
            IndicatorError::ImproperDataLength
        ));
    }
}
