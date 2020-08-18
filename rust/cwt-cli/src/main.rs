#![feature(test)] // for benchmarks.

extern crate test; // for benchmarks.

#[macro_use]
extern crate approx;

mod analysis;
mod conv;
mod cwt;
mod iter;

use std::fs::File;
use std::path::Path;

use cwt::alg;
use cwt::alg::Cwt;
use cwt::wavelets;

fn get_data(duration: f32) -> Option<(Vec<f32>, u32)> {
    let input_file = Path::new("data.wav");
    let mut inp_file = File::open(input_file).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    let fs = header.sampling_rate;

    // Remap to range -1.0 to 1.0, and take only 1000ms
    if let wav::BitDepth::Sixteen(raw_signal) = data {
        println!("Read success (i16)");
        let y = raw_signal
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .take((duration * fs as f32).floor() as usize)
            .collect();
        Some((y, fs))
    } else {
        None
    }
}

fn main() {
    let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
    let output_file = Path::new("scaleogram.csv");

    if let Some((d, fs)) = get_data(0.100) {
        println!("Signal length {}",d.len());
        println!("Sample rate {}",fs);
        let mut y = d.into_iter();

        // Do cwt
        let mut cwt = alg::FftCpx::new(|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
        let mut s = cwt.process(&mut y);
        analysis::threshold(&mut s, 100.);

        // Write cwt data to a file
        let mut wtr = csv::Writer::from_path(output_file).unwrap();
        for row in s.into_iter() {
            let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
            wtr.write_record(&text_vec).unwrap();
        }
        wtr.flush().unwrap();
    }
}
