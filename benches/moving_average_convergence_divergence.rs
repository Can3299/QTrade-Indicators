#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "macd")]
#[bench]
fn bench_macd_default_100(b: &mut Bencher) {
    let data: Vec<f64> = (0..100).map(|i| 10.0 + (i as f64).sin()).collect();
    let setting = indicator::moving_average_convergence_divergence::SettingMacd {
        fast_length: 12,
        slow_length: 26,
        signal_smooth: 9,
    };
    b.iter(|| {
        let _ = indicator::moving_average_convergence_divergence::calculate_macd(&data, &setting);
    });
}

#[cfg(feature = "macd")]
#[bench]
fn bench_macd_default_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| 10.0 + (i as f64).sin()).collect();
    let setting = indicator::moving_average_convergence_divergence::SettingMacd {
        fast_length: 12,
        slow_length: 26,
        signal_smooth: 9,
    };
    b.iter(|| {
        let _ = indicator::moving_average_convergence_divergence::calculate_macd(&data, &setting);
    });
}

#[cfg(feature = "macd")]
#[bench]
fn bench_macd_default_10000(b: &mut Bencher) {
    let data: Vec<f64> = (0..10000).map(|i| 10.0 + (i as f64).sin()).collect();
    let setting = indicator::moving_average_convergence_divergence::SettingMacd {
        fast_length: 12,
        slow_length: 26,
        signal_smooth: 9,
    };
    b.iter(|| {
        let _ = indicator::moving_average_convergence_divergence::calculate_macd(&data, &setting);
    });
}
