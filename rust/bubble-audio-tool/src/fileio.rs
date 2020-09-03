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

// Write scaleogram data to a csv file
pub fn export_scaleogram(s: &[Vec<f32>], dir: &Path, idx: usize) {
    let name = format!("scaleogram{}.csv", idx);
    let path = dir.join(Path::new(&name));

    let mut wtr = csv::Writer::from_path(path).unwrap();
    for row in s.iter() {
        let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
        wtr.write_record(&text_vec).unwrap();
    }
    wtr.flush().unwrap();
}

// Write bubble identification data to a csv file
pub fn export_bubble_data(b: &[(f32, f32)], dir: &Path, idx: usize) {
    let name = format!("bubbles{}.csv", idx);
    let path = dir.join(Path::new(&name));

    if !b.is_empty() {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        let text_vec: Vec<String> = b.iter().map(|(rad, _)| format!("{:e}", rad)).collect();
        wtr.write_record(&text_vec).unwrap();
        let text_vec: Vec<String> = b.iter().map(|(_, ts)| format!("{:e}", ts)).collect();
        wtr.write_record(&text_vec).unwrap();
        wtr.flush().unwrap();
    } else {
        File::create(path).unwrap();
    }
}
