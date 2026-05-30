//! Median Price indicator: `(high + low) / 2`.
/*!
A building-block indicator used by Supertrend. Gated behind `#[cfg(feature = "median_price")]`.

# Example

```rust
use qtrade_indicators::median_price::calculate_median_price;

let high = vec![10.0, 20.0, 30.0];
let low  = vec![5.0, 15.0, 25.0];
let result = calculate_median_price(&high, &low).unwrap();
assert_eq!(result, vec![7.5, 17.5, 27.5]);
```
*/

use crate::indicator_error::IndicatorError;
use tracing::instrument;

#[instrument(level = "trace", skip_all, ret)]
pub fn calculate_median_price(
    candle_high: &[f64],
    candle_low: &[f64],
) -> Result<Vec<f64>, IndicatorError> {
    if candle_high.is_empty() || candle_low.is_empty() {
        return Err(IndicatorError::EmptyData);
    }
    if candle_high.len() != candle_low.len() {
        return Err(IndicatorError::DifferentDataLength);
    }

    let median_price: Vec<f64> = candle_high
        .iter()
        .zip(candle_low.iter())
        .map(|(&high, &low)| (high + low) / 2.0)
        .collect();

    Ok(median_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_median_price_valid() {
        let high = vec![10.0, 20.0, 30.0];
        let low = vec![5.0, 15.0, 25.0];
        let result = calculate_median_price(&high, &low).unwrap();
        assert_eq!(result, vec![7.5, 17.5, 27.5]);
    }

    #[test]
    fn test_calculate_median_price_single_element() {
        let high = vec![42.0];
        let low = vec![38.0];
        let result = calculate_median_price(&high, &low).unwrap();
        assert_eq!(result, vec![40.0]);
    }

    #[test]
    fn test_calculate_median_price_empty_high() {
        let result = calculate_median_price(&[], &[1.0]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IndicatorError::EmptyData));
    }

    #[test]
    fn test_calculate_median_price_empty_low() {
        let result = calculate_median_price(&[1.0], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_median_price_different_lengths() {
        let result = calculate_median_price(&[1.0, 2.0], &[1.0]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            IndicatorError::DifferentDataLength
        ));
    }

    #[test]
    fn test_calculate_median_price_negative_values() {
        let high = vec![-10.0, 0.0];
        let low = vec![-20.0, -5.0];
        let result = calculate_median_price(&high, &low).unwrap();
        assert_eq!(result, vec![-15.0, -2.5]);
    }

    #[test]
    fn test_calculate_median_price_floating_precision() {
        let high = vec![3.333_333_3];
        let low = vec![1.111_111_1];
        let result = calculate_median_price(&high, &low).unwrap();
        let expected = (3.333_333_3 + 1.111_111_1) / 2.0;
        assert!((result[0] - expected).abs() < 1e-10);
    }
}
