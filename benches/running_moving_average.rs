#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "rma")]
#[bench]
fn bench_rma_period_14_size_1000(b: &mut Bencher) {
    let data: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let setting = indicator::running_moving_average::SettingRma { period: 14 };
    b.iter(|| {
        let _ = indicator::running_moving_average::calculate_rma(&data, &setting);
    });
}
