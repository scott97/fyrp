use std::fs::File;
use std::path::Path;

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