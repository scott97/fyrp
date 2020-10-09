#![feature(test)]
extern crate test;

use std::fs::File;
use std::path::Path;
use bubble_lib::*;
use test::Bencher;

pub fn get_data(path: &Path) -> Option<(Vec<f32>, u32)> {
    let mut input_file = File::open(path).unwrap();
    let (header, data) = wav::read(&mut input_file).unwrap();
    let fs = header.sampling_rate;

    // Remap to range -1.0 to 1.0
    if let wav::BitDepth::Sixteen(raw_signal) = data {
        let y = raw_signal
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect();
        Some((y, fs))
    } else {
        None
    }
}

pub fn bench_cwt(b: &mut Bencher, opts: &config::Opts) {
    #[rustfmt::skip]
    let (d, fs) = get_data(Path::new("test_data/data.wav"))
                        .expect("Could not find test data");

    const PEAK_FINDING_OVERLAP: usize = 2;
    let take = (opts.segment_size * 1e-3 * fs as f32) as usize;
    let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;

    let mut identifier = analysis::BubbleIdentifier::new(&opts, fs);

    // Repeat the test on the same 200ms (plus overlap) chunk
    let chunk = &d[..(take+peek)];

    b.iter(|| { 
        identifier.cwt(chunk.to_vec());
    });
}