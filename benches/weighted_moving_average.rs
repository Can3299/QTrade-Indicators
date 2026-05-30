#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "wma")]
#[bench]
fn bench_wma_period_14_size_100(b: &mut Bencher) {
    let data: Vec<f64> = (0..100).map(|i| (i as f64).sin()).collect();
    let setting = indicator::weighted_moving_average::SettingWma { period: 14 };
    b.iter(|| {
        let _ = indicator::weighted_moving_average::calculate_wma(&data, &setting);
    });
}

#[cfg(feature = "wma")]
#[bench]
fn bench_wma_period_14_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::weighted_moving_average::SettingWma { period: 14 };
    b.iter(|| {
        let _ = indicator::weighted_moving_average::calculate_wma(&data, &setting);
    });
}
