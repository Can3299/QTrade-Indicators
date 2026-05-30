#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "tr")]
#[bench]
fn bench_tr_100(b: &mut Bencher) {
    let close: Vec<f64> = (0..100).map(|i| (i as f64).sin()).collect();
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    b.iter(|| {
        let _ = indicator::true_range::calculate_tr(&close, &high, &low);
    });
}

#[cfg(feature = "tr")]
#[bench]
fn bench_tr_1000(b: &mut Bencher) {
    let close: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    b.iter(|| {
        let _ = indicator::true_range::calculate_tr(&close, &high, &low);
    });
}

#[cfg(feature = "tr")]
#[bench]
fn bench_tr_10000(b: &mut Bencher) {
    let close: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
    let high: Vec<f64> = close.iter().map(|c| c + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|c| c - 1.0).collect();
    b.iter(|| {
        let _ = indicator::true_range::calculate_tr(&close, &high, &low);
    });
}
