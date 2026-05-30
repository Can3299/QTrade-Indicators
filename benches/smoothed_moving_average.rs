#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "smma")]
#[bench]
fn bench_smma_period_14_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::smoothed_moving_average::SettingSmma { period: 14 };
    b.iter(|| {
        let _ = indicator::smoothed_moving_average::calculate_smma(&data, &setting);
    });
}

#[cfg(feature = "smma")]
#[bench]
fn bench_smma_period_14_size_10000(b: &mut Bencher) {
    let data: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::smoothed_moving_average::SettingSmma { period: 14 };
    b.iter(|| {
        let _ = indicator::smoothed_moving_average::calculate_smma(&data, &setting);
    });
}
