#[macro_use]
extern crate exec_time;

#[macro_use]
extern crate approx;

use std::fs::File;
use std::path::Path;

mod analysis;
mod conv;
mod cwt;
mod iter;

use cwt::wavelets::soulti_cpx;
use cwt::alg::Cwt;

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

            // Remap to range -1.0 to 1.0, and take only 1000ms
            let mut y = raw_signal
                .iter()
                .map(|x| (*x as f32) / (i16::MAX as f32))
                .take((1.000* fs as f32) as usize);

            // Frequencies (1 to 9 kHz at interval of 20Hz)
            let frequencies: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();

            // Do cwt
            // let mut cwt = cwt::alg::FftCpxFilterBank::new(1.000, |t| soulti_cpx(t, 0.02), [0.0, 50.0], &frequencies, fs);
            let mut cwt = cwt::alg::FftCpx::new(|t| soulti_cpx(t, 0.02), [0.0, 50.0], &frequencies, fs);
            let mut s = cwt.process_par(&mut y);
            analysis::threshold(&mut s, 100.);

            // Write cwt data to a file
            let mut wtr = csv::Writer::from_path(output_file).unwrap();
            for row in s.into_iter() {
                let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
                wtr.write_record(&text_vec).unwrap();
            }
            wtr.flush().unwrap();

            // Benchmark cwt variants
            // let mut cwt = cwt::alg::FftCpx::new(|t| soulti_cpx(t, 0.02), [0.0, 50.0], &frequencies, fs);
            // let mut s = cwt.process_par(&y);
            // let mut cwt = cwt::alg::FftCpxFilterBank::new(1.000, |t| soulti_cpx(t, 0.02), [0.0, 50.0], &frequencies, fs);
            // let mut s = cwt.process_par(&y);
        }
        _ => panic!("read error or wrong wave type"),
    }
}
