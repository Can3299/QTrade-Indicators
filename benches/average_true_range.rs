#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "atr")]
#[bench]
fn bench_atr_sma_period_14_size_1000(b: &mut Bencher) {
    let close: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    let setting = indicator::average_true_range::SettingAtr {
        period: 14,
        smooth_engine: indicator::average_true_range::SmoothEngine::SMA,
    };
    b.iter(|| {
        let _ = indicator::average_true_range::calculate_atr(&close, &high, &low, &setting);
    });
}

#[cfg(feature = "atr")]
#[bench]
fn bench_atr_ema_period_14_size_1000(b: &mut Bencher) {
    let close: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    let setting = indicator::average_true_range::SettingAtr {
        period: 14,
        smooth_engine: indicator::average_true_range::SmoothEngine::EMA,
    };
    b.iter(|| {
        let _ = indicator::average_true_range::calculate_atr(&close, &high, &low, &setting);
    });
}
