//! Exponential Moving Average (EMA) — exponentially weighted moving average.
/*!
Applies exponentially decreasing weights to older observations.
The weighting factor is `alpha = 2 / (period + 1)`.

Used by MACD (fast/slow EMAs and signal line) and ATR (when engine is `SmoothEngine::EMA`).

Gated behind `#[cfg(feature = "ema")]`.

# Algorithm

```text
alpha = 2 / (period + 1)

EMA[i] = 0.0                                      for i < period - 1
EMA[period-1] = SMA(data[0..period])               (seed with simple average)
EMA[i] = alpha * data[i] + (1 - alpha) * EMA[i-1]  for i >= period
```

# Example

```rust
use qtrade_indicators::exponential_moving_average::{SettingEma, calculate_ema};

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let setting = SettingEma { period: 3 };
let ema = calculate_ema(&data, &setting).unwrap();
// ema = [0.0, 0.0, 2.0, 3.0, 4.0]  (alpha = 0.5)
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// Configuration for Exponential Moving Average calculation.
pub struct SettingEma {
    /// Lookback window size. Must be >= 1 and strictly less than data length.
    pub period: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_ema(
    candle_data: &[f64],
    setting: &SettingEma,
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

    let mut ema: Vec<f64> = Vec::with_capacity(candle_data.len());
    let period_f = setting.period as f64;
    let alpha: f64 = 2.0 / (period_f + 1.0);
    let inv_alpha = 1.0 - alpha;

    // First Period
    ema.extend(std::iter::repeat_n(0.0, setting.period - 1));

    // First EMA
    let mut last_ema: f64 =
        candle_data.iter().take(setting.period).sum::<f64>() / setting.period as f64;
    ema.push(last_ema);

    for &price in candle_data.iter().skip(setting.period) {
        let current_ema = alpha.mul_add(price, inv_alpha * last_ema);

        last_ema = current_ema;
        ema.push(current_ema);
    }

    Ok(ema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ema_known_values() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let setting = SettingEma { period: 3 };
        let result = calculate_ema(&data, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 0.0);
        // First EMA = SMA of first 3: (1+2+3)/3 = 2.0
        assert!((result[2] - 2.0).abs() < 1e-10);
        // alpha = 2/(3+1) = 0.5
        // EMA_4 = 0.5*4 + 0.5*2.0 = 3.0
        assert!((result[3] - 3.0).abs() < 1e-10);
        // EMA_5 = 0.5*5 + 0.5*3.0 = 4.0
        assert!((result[4] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_ema_period_one() {
        let data = vec![10.0, 20.0, 30.0];
        let setting = SettingEma { period: 1 };
        let result = calculate_ema(&data, &setting).unwrap();
        assert_eq!(result.len(), 3);
        // period-1 = 0 leading zeros, first EMA = SMA of first 1 = 10.0
        // alpha = 2/(1+1) = 1.0
        assert!((result[0] - 10.0).abs() < 1e-10);
        // EMA_2 = 1.0*20 + 0.0*10 = 20
        assert!((result[1] - 20.0).abs() < 1e-10);
        assert!((result[2] - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_ema_constant_data() {
        let data = vec![50.0; 10];
        let setting = SettingEma { period: 5 };
        let result = calculate_ema(&data, &setting).unwrap();
        for &val in result.iter().skip(setting.period - 1) {
            assert!((val - 50.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_calculate_ema_decreasing() {
        let data = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let setting = SettingEma { period: 3 };
        let result = calculate_ema(&data, &setting).unwrap();
        // alpha = 0.5, First = (10+8+6)/3 = 8.0
        // 0.5*4 + 0.5*8 = 6.0
        // 0.5*2 + 0.5*6 = 4.0
        for &val in &result {
            assert!(val <= 8.0);
        }
    }

    #[test]
    fn test_calculate_ema_empty() {
        assert!(matches!(
            calculate_ema(&[], &SettingEma { period: 5 }).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_ema_zero_period() {
        assert!(matches!(
            calculate_ema(&[1.0], &SettingEma { period: 0 }).unwrap_err(),
            IndicatorError::ImproperSetting
        ));
    }
}
