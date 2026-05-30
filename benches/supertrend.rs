#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "supertrend")]
#[bench]
fn bench_supertrend_factor_3_size_1000(b: &mut Bencher) {
    let close: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let median: Vec<f64> = close.iter().map(|c| c * 0.5).collect();
    let atr: Vec<f64> = (0..1000).map(|_| 1.0).collect();
    let setting = indicator::supertrend::SettingSupertrend { factor: 3.0 };
    b.iter(|| {
        let _ = indicator::supertrend::calculate_supertrend(&close, &median, &atr, &setting);
    });
}
