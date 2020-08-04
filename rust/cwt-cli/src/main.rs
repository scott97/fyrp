#[macro_use]
extern crate exec_time;

#[macro_use]
extern crate approx;

use std::fs::File;
use std::path::Path;

mod conv;
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
            let frequencies: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();

            // Do cwt
            let s = wavelets::cwt_par_simd(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);

            // Write cwt data to a file
            let mut wtr = csv::Writer::from_path(output_file).unwrap();
            for row in s.into_iter() {
                // let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
                let text_vec: Vec<String> = row.iter().map(|n| format!("{}", n)).collect();
                wtr.write_record(&text_vec).unwrap();
            }
            wtr.flush().unwrap();

            // Benchmark cwt variants (10 Hz apart)
            {
                let frequencies: Vec<f32> = iter::rangef(1000.0, 9000.0, 10.0).collect();
                wavelets::cwt_par_simd(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_par_fft(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_par(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_simd(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_fft(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);

                // to stop overflowing, I am using 8 bits for the signal and 2 bits for the wavelet
                let yi16: Vec<i16> = y.iter().map(|x| (*x * 256.) as i16).collect();
                wavelets::cwt_par_simd_i16(
                    &yi16,
                    |t| (wavelets::soulti(t, 0.02) * 4.) as i16,
                    wvlt_bounds,
                    &frequencies,
                    fs,
                );
            }

            // Compare lower amount of frequencies (20Hz apart so half as many bands)
            // Here I halfed it, will i get double performance?
            // Will resolution still be sufficient?
            // Should frequencies be linearly spaced apart? Or maybe proportional to surface area of bubbles, so that error is constant across bands.
            {
                let frequencies: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
                wavelets::cwt_par_simd(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_par_fft(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_par(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_simd(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt_fft(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);
                wavelets::cwt(&y, wvlt_fn, wvlt_bounds, &frequencies, fs);

                // to stop overflowing, I am using 8 bits for the signal and 2 bits for the wavelet
                let yi16: Vec<i16> = y.iter().map(|x| (*x * 256.) as i16).collect();
                wavelets::cwt_par_simd_i16(
                    &yi16,
                    |t| (wavelets::soulti(t, 0.02) * 4.) as i16,
                    wvlt_bounds,
                    &frequencies,
                    fs,
                );
            }
        }
        _ => panic!("read error or wrong wave type"),
    }
}
