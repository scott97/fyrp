#[macro_use]
extern crate exec_time;

use std::fs::File;
use std::path::Path;

mod iter;
mod wavelets;

fn main() {
    let input_file = Path::new("data.wav");
    let output_file = Path::new("scaleogram.csv");

    println!("CWT implementation that can be put on a microcontroller");

    // Get signal y(t)
    let mut inp_file = File::open(input_file).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    let fs = header.sampling_rate;

    match data {
        wav::BitDepth::Sixteen(raw_signal) => {
            println!("read success (i16), length {:?}", raw_signal.len());

            // Remap to range -1.0 to 1.0, and take only 100ms
            let y = raw_signal
                .iter()
                .map(|x| (*x as f32) / (i16::MAX as f32))
                .take((0.100 * fs as f32) as usize)
                .collect::<Vec<f32>>();

            // Wavelet function
            let wvlt_fn = |t| wavelets::soulti(t, 0.02);
            let wvlt_bounds = [0.0, 50.0];

            // Frequencies (1 to 9 kHz at interval of 10Hz)
            let frequencies: Vec<f32> = iter::rangef(1000.0, 9000.0, 10.0).collect();

            // Do cwt
            let s = wavelets::cwt_par(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);

            // Write cwt data to a file
            let mut wtr = csv::Writer::from_path(output_file).unwrap();
            for row in s.into_iter() {
                let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
                wtr.write_record(&text_vec).unwrap();
            }
            wtr.flush().unwrap();

            // Measure the other cwts
            wavelets::cwt(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);

        }
        _ => panic!("read error or wrong wave type"),
    }
}
