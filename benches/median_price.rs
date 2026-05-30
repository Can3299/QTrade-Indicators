#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "median_price")]
#[bench]
fn bench_median_price_100(b: &mut Bencher) {
    let high: Vec<f64> = (0..100).map(|i| i as f64).collect();
    let low: Vec<f64> = (0..100).map(|i| (i as f64) - 1.0).collect();
    b.iter(|| {
        let _ = indicator::median_price::calculate_median_price(&high, &low);
    });
}

#[cfg(feature = "median_price")]
#[bench]
fn bench_median_price_1000(b: &mut Bencher) {
    let high: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    let low: Vec<f64> = (0..1000).map(|i| (i as f64) - 1.0).collect();
    b.iter(|| {
        let _ = indicator::median_price::calculate_median_price(&high, &low);
    });
}

#[cfg(feature = "median_price")]
#[bench]
fn bench_median_price_10000(b: &mut Bencher) {
    let high: Vec<f64> = (0..10000).map(|i| i as f64).collect();
    let low: Vec<f64> = (0..10000).map(|i| (i as f64) - 1.0).collect();
    b.iter(|| {
        let _ = indicator::median_price::calculate_median_price(&high, &low);
    });
}
