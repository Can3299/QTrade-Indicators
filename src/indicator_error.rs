//! Shared error type used uniformly by all indicator calculation functions.
/*!
Defines the [`IndicatorError`] enum returned by every `calculate_*` function
in this crate.

| Variant | Condition | Returned by |
|---------|-----------|-------------|
| `EmptyData` | Any input slice is empty | All `calculate_*` functions |
| `DifferentDataLength` | Multiple input slices have mismatched lengths | `calculate_median_price`, `calculate_tr`, `calculate_atr`, `calculate_supertrend`, `calculate_wf` |
| `ImproperDataLength` | Input data length is insufficient for the computation period | All MA functions, `calculate_atr`, `calculate_wf` |
| `ImproperSetting` | Configuration is invalid (e.g. zero period, zero/negative factor) | All `calculate_*` functions that accept a `Setting*` struct |

# Example

```rust
use qtrade_indicators::indicator_error::IndicatorError;

fn check_length(data: &[f64], period: usize) -> Result<(), IndicatorError> {
    if data.len() <= period {
        return Err(IndicatorError::ImproperDataLength);
    }
    Ok(())
}
```
*/

use std::fmt;
use std::error::Error;

/// Errors returned by indicator calculation functions.
#[derive(Debug)]
pub enum IndicatorError {
    /// One or more input slices are empty.
    EmptyData,
    /// Multiple input slices have mismatched lengths.
    DifferentDataLength,
    /// Input data is too short for the requested period / factor.
    ImproperDataLength,
    /// Configuration is invalid (e.g. zero period, zero/negative factor).
    ImproperSetting,
}
impl fmt::Display for IndicatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            IndicatorError::EmptyData => "Input data can not be empty",
            IndicatorError::DifferentDataLength => {
                "Input data can not have a different length than input"
            }
            IndicatorError::ImproperDataLength => "Improper data length",
            IndicatorError::ImproperSetting => "Improper setting",
        };

        write!(f, "{}", s)
    }
}
impl Error for IndicatorError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_error_display_empty_data() {
        let err = IndicatorError::EmptyData;
        assert_eq!(err.to_string(), "Input data can not be empty");
    }

    #[test]
    fn test_indicator_error_display_different_length() {
        let err = IndicatorError::DifferentDataLength;
        assert_eq!(
            err.to_string(),
            "Input data can not have a different length than input"
        );
    }

    #[test]
    fn test_indicator_error_display_improper_length() {
        let err = IndicatorError::ImproperDataLength;
        assert_eq!(err.to_string(), "Improper data length");
    }

    #[test]
    fn test_indicator_error_display_improper_setting() {
        let err = IndicatorError::ImproperSetting;
        assert_eq!(err.to_string(), "Improper setting");
    }

    #[test]
    fn test_indicator_error_implements_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<IndicatorError>();
    }
}
