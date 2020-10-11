mod test_utils;

use bubble_lib::*;
use std::path::Path;
use std::thread;

#[test]
fn integration() {
    let opts = config::Opts {
        // Params which I am benchmarking performance with.
        cwt: config::CwtAlg::FftFilterBank,
        wavelet_type: config::WaveletType::CplxWavelet,
        parallel: true,
        radius_resolution: 0.02, // mm
        segment_size: 200.,      // ms

        // Params which remain the same.
        debug: false,
        threshold: 100.,
        threshold_type: config::ThresholdType::ProportionalToRadius,
        min_radius: 0.30, // mm
        max_radius: 3.00, // mm
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    };

    #[rustfmt::skip]
    let (d, fs) = test_utils::get_data(Path::new("test_data/data.wav"))
                                    .expect("Could not find test data");

    const PEAK_FINDING_OVERLAP: usize = 2;
    let take = (opts.segment_size * 1e-3 * fs as f32) as usize;
    let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;

    let (mut send, mut recv) = segmenter::Segmenter::split(take, peek);

    let t = thread::spawn(move || {
        for val in d.into_iter() {
            send.push(val);
        }
    });
    t.join().unwrap();

    let mut identifier = analysis::BubbleIdentifier::new(&opts, fs);

    while let Ok(chunk) = recv.pop_segment() {
        identifier.cwt(chunk);
    }
}
