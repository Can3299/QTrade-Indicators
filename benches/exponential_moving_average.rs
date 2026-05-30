#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "ema")]
#[bench]
fn bench_ema_period_14_size_100(b: &mut Bencher) {
    let data: Vec<f64> = (0..100).map(|i| (i as f64).sin()).collect();
    let setting = indicator::exponential_moving_average::SettingEma { period: 14 };
    b.iter(|| {
        let _ = indicator::exponential_moving_average::calculate_ema(&data, &setting);
    });
}

#[cfg(feature = "ema")]
#[bench]
fn bench_ema_period_14_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::exponential_moving_average::SettingEma { period: 14 };
    b.iter(|| {
        let _ = indicator::exponential_moving_average::calculate_ema(&data, &setting);
    });
}

#[cfg(feature = "ema")]
#[bench]
fn bench_ema_period_14_size_10000(b: &mut Bencher) {
    let data: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::exponential_moving_average::SettingEma { period: 14 };
    b.iter(|| {
        let _ = indicator::exponential_moving_average::calculate_ema(&data, &setting);
    });
}
