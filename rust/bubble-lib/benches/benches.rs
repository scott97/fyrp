#![feature(test)]
extern crate test;

mod test_utils;
use bubble_lib::*;
use test::Bencher;
use config::Opts;


// Comparing Algorithms
#[bench]
fn bench_cwt_fft_filterbank(mut b: &mut Bencher) {
    test_utils::bench_cwt(&mut b, &Opts {
        // Variables
        cwt: config::CwtAlg::FftFilterBank,

        // Experimental Constants
        wavelet_type: config::WaveletType::CplxWavelet,
        parallel: true,
        radius_resolution: 0.02, // mm
        segment_size: 200.,      // ms

        // More Experimental Constants
        debug: false,
        threshold: 100.,
        min_radius: 0.30, // mm
        max_radius: 3.00, // mm
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    });
}


#[bench]
fn bench_cwt_fft(mut b: &mut Bencher) {
    test_utils::bench_cwt(&mut b, &Opts {
        // Variables
        cwt: config::CwtAlg::Fft,

        // Experimental Constants
        wavelet_type: config::WaveletType::CplxWavelet,
        parallel: true,
        radius_resolution: 0.02, // mm
        segment_size: 200.,      // ms

        // More Experimental Constants
        debug: false,
        threshold: 100.,
        min_radius: 0.30, // mm
        max_radius: 3.00, // mm
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    });
}

#[bench]
fn bench_cwt_simd(mut b: &mut Bencher) {
    test_utils::bench_cwt(&mut b, &Opts {
        // Variables
        cwt: config::CwtAlg::Simd,

        // Experimental Constants
        wavelet_type: config::WaveletType::CplxWavelet,
        parallel: true,
        radius_resolution: 0.02, // mm
        segment_size: 200.,      // ms

        // More Experimental Constants
        debug: false,
        threshold: 100.,
        min_radius: 0.30, // mm
        max_radius: 3.00, // mm
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    });
}


#[bench]
fn bench_cwt_standard(mut b: &mut Bencher) {
    test_utils::bench_cwt(&mut b, &Opts {
        // Variables
        cwt: config::CwtAlg::Standard,

        // Experimental Constants
        wavelet_type: config::WaveletType::CplxWavelet,
        parallel: true,
        radius_resolution: 0.02, // mm
        segment_size: 200.,      // ms

        // More Experimental Constants
        debug: false,
        threshold: 100.,
        min_radius: 0.30, // mm
        max_radius: 3.00, // mm
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    });
}
