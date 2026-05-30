//! True Range (TR) — a volatility measure.
/*!
Computes the True Range for each candle, defined as the greatest of:
- Current high minus current low
- Absolute value of current high minus previous close
- Absolute value of current low minus previous close

Gated behind `#[cfg(feature = "tr")]`.

# Algorithm

```text
TR[0] = high[0] - low[0]
TR[i] = max(high[i] - low[i], |high[i] - close[i-1]|, |low[i] - close[i-1]|)   for i >= 1
```

# Example

```rust
use qtrade_indicators::true_range::calculate_tr;

let close = vec![10.0, 12.0, 11.0];
let high  = vec![11.0, 13.0, 12.0];
let low   = vec![9.0, 11.0, 10.0];
let tr = calculate_tr(&close, &high, &low).unwrap();
// tr[0] = 2, tr[1] = 3, tr[2] = 2
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_tr(
    candle_close: &[f64],
    candle_high: &[f64],
    candle_low: &[f64],
) -> Result<Vec<f64>, IndicatorError> {
    if candle_close.is_empty() || candle_high.is_empty() || candle_low.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if candle_close.len() != candle_high.len() || candle_high.len() != candle_low.len() {
        return Err(IndicatorError::DifferentDataLength);
    }

    let mut tr: Vec<f64> = Vec::with_capacity(candle_close.len());

    // First candle
    tr.push(candle_high[0] - candle_low[0]);

    tr.extend(
        candle_high
            .iter()
            .skip(1)
            .zip(candle_low.iter().skip(1))
            .zip(candle_close.iter())
            .map(|((&high, &low), &prev_close)| {
                let h_l: f64 = high - low;
                let h_pc: f64 = (high - prev_close).abs();
                let l_pc: f64 = (low - prev_close).abs();

                h_l.max(h_pc).max(l_pc)
            }),
    );

    Ok(tr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tr_valid() {
        let close = vec![10.0, 12.0, 11.0];
        let high = vec![11.0, 13.0, 12.0];
        let low = vec![9.0, 11.0, 10.0];
        let result = calculate_tr(&close, &high, &low).unwrap();
        assert_eq!(result.len(), 3);
        // First: H-L = 11-9 = 2.0
        assert!((result[0] - 2.0).abs() < 1e-10);
        // Second: max(H-L=2, |H-pC|=|13-10|=3, |L-pC|=|11-10|=1) = 3
        assert!((result[1] - 3.0).abs() < 1e-10);
        // Third: max(H-L=2, |H-pC|=|12-12|=0, |L-pC|=|10-12|=2) = 2
        assert!((result[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_tr_all_gaps() {
        let close = vec![100.0, 110.0, 120.0];
        let high = vec![105.0, 115.0, 125.0];
        let low = vec![95.0, 105.0, 115.0];
        let result = calculate_tr(&close, &high, &low).unwrap();
        assert_eq!(result.len(), 3);
        // First: 105-95 = 10
        assert!((result[0] - 10.0).abs() < 1e-10);
        // Second: max(115-105=10, |115-100|=15, |105-100|=5) = 15
        assert!((result[1] - 15.0).abs() < 1e-10);
        // Third: max(125-115=10, |125-110|=15, |115-110|=5) = 15
        assert!((result[2] - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_tr_empty() {
        assert!(matches!(
            calculate_tr(&[], &[1.0], &[1.0]).unwrap_err(),
            IndicatorError::EmptyData
        ));
    }

    #[test]
    fn test_calculate_tr_length_mismatch() {
        assert!(matches!(
            calculate_tr(&[1.0, 2.0], &[1.0], &[1.0]).unwrap_err(),
            IndicatorError::DifferentDataLength
        ));
    }

    #[test]
    fn test_calculate_tr_single_element() {
        let result = calculate_tr(&[10.0], &[12.0], &[8.0]).unwrap();
        assert_eq!(result.len(), 1);
        assert!((result[0] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_tr_abs_used_correctly() {
        let close = vec![50.0, 40.0];
        let high = vec![55.0, 42.0];
        let low = vec![45.0, 38.0];
        let result = calculate_tr(&close, &high, &low).unwrap();
        // First: 55-45 = 10
        assert!((result[0] - 10.0).abs() < 1e-10);
        // Second: max(42-38=4, |42-50|=8, |38-50|=12) = 12
        assert!((result[1] - 12.0).abs() < 1e-10);
    }
}
