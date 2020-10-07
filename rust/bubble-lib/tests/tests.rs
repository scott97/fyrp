mod common;
mod data;

use bubble_lib::*;
use std::path::Path;

#[test]
fn integration_test() {
    let opts = config::Opts {
        cwt: config::CwtAlg::FftFilterBank,
        debug: false,
        segment_size: 200., // ms
        threshold: 100.,
        radius_resolution: 0.02, // mm
        min_radius: 0.30,        // mm
        max_radius: 3.00,        // mm
        parallel: true,
        clustering: true,
        clustering_window: config::ClusteringWindow::Circular,
        clustering_window_bandwidths: vec![15.],
        max_iterations: 20,
        wavelet: config::Wavelet::Laplace,
        zeta: 0.02,
    };

    let (d, fs) = common::get_data(Path::new("tests/data/data.wav")).unwrap();

    // Chunk length requirements.
    const PEAK_FINDING_OVERLAP: usize = 2;
    let take = (opts.segment_size * 1e-3 * fs as f32) as usize;
    let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;

    let mut iter = d.into_iter().peekable();
    let mut identifier = analysis::BubbleIdentifier::new(&opts, fs);
    let mut joiner = summary::Joiner::new(&opts);

    // Read in data chunk by chunk (with overlap).
    'outer: for i in 1.. {
        let mut chunk = Vec::with_capacity(take + peek);
        for _j in 0..take {
            match iter.next() {
                Some(x) => chunk.push(x),
                None => break 'outer,
            }
        }
        for _j in 0..peek {
            match iter.peek() {
                Some(x) => chunk.push(*x),
                None => break 'outer,
            }
        }

        // Process chunk
        let mut s = identifier.cwt(chunk);
        identifier.threshold(&mut s);
        let b = identifier.find_bubbles(&s);
        joiner.append(i, &b);
    }

    let expected = data::get();
    let actual = joiner.get_joined();

    assert_eq!(expected, actual);
}
