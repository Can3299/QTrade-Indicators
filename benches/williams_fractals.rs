#![feature(test)]

extern crate test;
use test::Bencher;

#[cfg(feature = "wf")]
#[bench]
fn bench_wf_factor_2_size_100(b: &mut Bencher) {
    let high: Vec<f64> = (0..100).map(|i| (i as f64).sin()).collect();
    let low: Vec<f64> = high.iter().map(|h| h - 1.0).collect();
    let setting = indicator::williams_fractals::SettingWf { factor: 2 };
    b.iter(|| {
        let _ = indicator::williams_fractals::calculate_wf(&high, &low, &setting);
    });
}

#[cfg(feature = "wf")]
#[bench]
fn bench_wf_factor_2_size_1000(b: &mut Bencher) {
    let high: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let low: Vec<f64> = high.iter().map(|h| h - 1.0).collect();
    let setting = indicator::williams_fractals::SettingWf { factor: 2 };
    b.iter(|| {
        let _ = indicator::williams_fractals::calculate_wf(&high, &low, &setting);
    });
}

#[cfg(feature = "wf")]
#[bench]
fn bench_wf_factor_5_size_1000(b: &mut Bencher) {
    let high: Vec<f64> = (0..1000).map(|i| (i as f64).sin()).collect();
    let low: Vec<f64> = high.iter().map(|h| h - 1.0).collect();
    let setting = indicator::williams_fractals::SettingWf { factor: 5 };
    b.iter(|| {
        let _ = indicator::williams_fractals::calculate_wf(&high, &low, &setting);
    });
}

#[cfg(feature = "wf")]
#[bench]
fn bench_wf_factor_2_size_10000(b: &mut Bencher) {
    let high: Vec<f64> = (0..10000).map(|i| (i as f64).sin()).collect();
    let low: Vec<f64> = high.iter().map(|h| h - 1.0).collect();
    let setting = indicator::williams_fractals::SettingWf { factor: 2 };
    b.iter(|| {
        let _ = indicator::williams_fractals::calculate_wf(&high, &low, &setting);
    });
}
