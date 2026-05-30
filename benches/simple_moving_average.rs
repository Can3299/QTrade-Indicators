#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "sma")]
#[bench]
fn bench_sma_period_14_size_100(b: &mut Bencher) {
    let data: Vec<f64> = (0..100).map(|i| (i as f64).sin()).collect();
    let setting = indicator::simple_moving_average::SettingSma { period: 14 };
    b.iter(|| {
        let _ = indicator::simple_moving_average::calculate_sma(&data, &setting);
    });
}

#[cfg(feature = "sma")]
#[bench]
fn bench_sma_period_14_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::simple_moving_average::SettingSma { period: 14 };
    b.iter(|| {
        let _ = indicator::simple_moving_average::calculate_sma(&data, &setting);
    });
}

#[cfg(feature = "sma")]
#[bench]
fn bench_sma_period_50_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::simple_moving_average::SettingSma { period: 50 };
    b.iter(|| {
        let _ = indicator::simple_moving_average::calculate_sma(&data, &setting);
    });
}

#[cfg(feature = "sma")]
#[bench]
fn bench_sma_period_14_size_10000(b: &mut Bencher) {
    let data: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::simple_moving_average::SettingSma { period: 14 };
    b.iter(|| {
        let _ = indicator::simple_moving_average::calculate_sma(&data, &setting);
    });
}
