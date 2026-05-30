//! Williams Fractals — local price extreme points for support/resistance.
/*!
Identifies fractal patterns consisting of a central candle with `factor` lower highs
on each side (high fractal) or `factor` higher lows on each side (low fractal).

Gated behind `#[cfg(feature = "wf")]`.

# Algorithm

For each candle `i` where `factor <= i < len - factor`:

```text
is_fractal_high = high[i] > high[i-j] AND high[i] > high[i+j] for all j in 1..=factor
is_fractal_low  = low[i]  < low[i-j]  AND low[i]  < low[i+j]  for all j in 1..=factor
```

# Example

```rust
use qtrade_indicators::williams_fractals::{SettingWf, calculate_wf};

let high = vec![10.0, 15.0, 20.0, 15.0, 10.0];
let low  = vec![5.0, 8.0, 10.0, 8.0, 5.0];
let setting = SettingWf { factor: 2 };

let wf = calculate_wf(&high, &low, &setting).unwrap();
assert!(wf[2].fractal_high);  // 20 > all neighbors
```
*/

use std::vec;

use crate::indicator_error::IndicatorError;
use tracing::instrument;

/// A single Williams Fractals data point.
#[derive(Debug, Clone)]
pub struct WFData {
    /// Whether this candle is a local high fractal (pivot high).
    pub fractal_high: bool,
    /// Whether this candle is a local low fractal (pivot low).
    pub fractal_low: bool,
}

/// Configuration for Williams Fractals calculation.
pub struct SettingWf {
    /// Number of candles required on each side of a fractal (e.g. 2 means 2 left + 2 right neighbors).
    pub factor: usize,
}

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_wf(
    candle_high: &[f64],
    candle_low: &[f64],
    setting: &SettingWf,
) -> Result<Vec<WFData>, IndicatorError> {
    if candle_high.is_empty() || candle_low.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if candle_high.len() != candle_low.len() {
        return Err(IndicatorError::DifferentDataLength);
    }
    if candle_high.len() < ((setting.factor * 2) + 1) {
        return Err(IndicatorError::ImproperDataLength);
    }

    let mut wf: Vec<WFData> = vec![
        WFData {
            fractal_high: false,
            fractal_low: false
        };
        candle_high.len()
    ];

    for i in setting.factor..(candle_high.len() - setting.factor) {
        let center_high = candle_high[i];
        let center_low = candle_low[i];

        let mut is_fractal_high = true;
        let mut is_fractal_low = true;

        for j in 1..=setting.factor {
            // High Fractal
            if center_high <= candle_high[i - j] || center_high <= candle_high[i + j] {
                is_fractal_high = false;
            }
            // Low Fractal
            if center_low >= candle_low[i - j] || center_low >= candle_low[i + j] {
                is_fractal_low = false;
            }

            if !is_fractal_high && !is_fractal_low {
                break;
            }
        }

        wf[i] = WFData {
            fractal_high: is_fractal_high,
            fractal_low: is_fractal_low,
        };
    }
    Ok(wf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_wf_high_fractal() {
        // Peak at index 2: 20 > left neighbors (10,15) and right neighbors (15,10)
        let high = vec![10.0, 15.0, 20.0, 15.0, 10.0];
        let low = vec![5.0, 8.0, 10.0, 8.0, 5.0];
        let setting = SettingWf { factor: 2 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert!(result[2].fractal_high);
        assert!(!result[2].fractal_low);
    }

    #[test]
    fn test_calculate_wf_low_fractal() {
        // Valley at index 2: 5 < left neighbors (10,8) and right neighbors (8,10)
        let high = vec![20.0, 15.0, 10.0, 15.0, 20.0];
        let low = vec![10.0, 8.0, 5.0, 8.0, 10.0];
        let setting = SettingWf { factor: 2 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        assert_eq!(result.len(), 5);
        assert!(result[2].fractal_low);
        assert!(!result[2].fractal_high);
    }

    #[test]
    fn test_calculate_wf_both_fractals() {
        // Both high peak and low valley at same index
        let high = vec![5.0, 8.0, 15.0, 8.0, 5.0];
        let low = vec![3.0, 4.0, 2.0, 4.0, 3.0];
        let setting = SettingWf { factor: 2 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        // Index 2: high=15 > all neighbors => fractal_high=true
        assert!(result[2].fractal_high);
        // Index 2: low=2 < all neighbors => fractal_low=true
        assert!(result[2].fractal_low);
    }

    #[test]
    fn test_calculate_wf_no_fractal() {
        // No clear peak or valley
        let high = vec![10.0, 12.0, 11.0, 13.0, 10.0];
        let low = vec![8.0, 9.0, 8.5, 10.0, 7.0];
        let setting = SettingWf { factor: 2 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        // Index 2: high=11 <= high[1]=12 => not fractal
        assert!(!result[2].fractal_high);
    }

    #[test]
    fn test_calculate_wf_boundary_elements() {
        let high = vec![1.0, 2.0, 3.0, 2.0, 1.0];
        let low = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let setting = SettingWf { factor: 2 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        // Elements outside the search range (indices < factor or > len-factor-1) must be false
        assert!(!result[0].fractal_high);
        assert!(!result[1].fractal_high);
        assert!(!result[3].fractal_high);
        assert!(!result[4].fractal_high);
    }

    #[test]
    fn test_calculate_wf_empty() {
        assert!(matches!(
            calculate_wf(&[], &[1.0], &SettingWf { factor: 2 }).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_wf_length_mismatch() {
        assert!(matches!(
            calculate_wf(&[1.0, 2.0], &[1.0], &SettingWf { factor: 2 }).unwrap_err(),
            IndicatorError::DifferentDataLength
        ));
    }

    #[test]
    fn test_calculate_wf_insufficient_data() {
        assert!(matches!(
            calculate_wf(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0], &SettingWf { factor: 2 }).unwrap_err(),
            IndicatorError::ImproperDataLength
        ));
    }

    #[test]
    fn test_calculate_wf_minimum_valid() {
        // factor=1 requires 1*2+1=3 elements minimum
        let high = vec![1.0, 3.0, 2.0];
        let low = vec![1.0, 1.0, 1.0];
        let setting = SettingWf { factor: 1 };
        let result = calculate_wf(&high, &low, &setting).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result[1].fractal_high);
    }
}
