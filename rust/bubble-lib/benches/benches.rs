#![feature(test)]
extern crate test;

mod test_utils;
use bubble_lib::*;
use config::Opts;
use test::Bencher;

// Comparing Algorithms (parallel performance)
#[bench]
fn bench_cwt_standard_parallel_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Standard,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_simd_parallel_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Simd,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_parallel_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Fft,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_filterbank_parallel_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::FftFilterBank,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

// Comparing algorithms (single threaded performance)
#[bench]
fn bench_cwt_standard_single_threaded_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Standard,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: false,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_simd_single_threaded_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Simd,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: false,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_single_threaded_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Fft,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: false,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_filterbank_single_threaded_cplx_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::FftFilterBank,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: false,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

// Demonstrating that complex wavelets have a performance impact on some algorithms
// Note: fft and fft_filterbank do not support real wavelets, only complex.
#[bench]
fn bench_cwt_standard_parallel_real_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Standard,
            wavelet_type: config::WaveletType::RealWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_simd_parallel_real_wvlt_20um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Simd,
            wavelet_type: config::WaveletType::RealWavelet,
            parallel: true,
            radius_resolution: 0.02, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

// Reducing radius resolution can improve performance.
// This is because it uses less frequency bands, which saves on calculations.
#[bench]
fn bench_cwt_standard_parallel_cplx_wvlt_40um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Standard,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.04, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_simd_parallel_cplx_wvlt_40um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Simd,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.04, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_parallel_cplx_wvlt_40um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::Fft,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.04, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}

#[bench]
fn bench_cwt_fft_filterbank_parallel_cplx_wvlt_40um(mut b: &mut Bencher) {
    test_utils::bench_cwt(
        &mut b,
        &Opts {
            // Variables
            cwt: config::CwtAlg::FftFilterBank,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.04, // mm

            // Experimental Constants
            segment_size: 200., // ms
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
        },
    );
}
