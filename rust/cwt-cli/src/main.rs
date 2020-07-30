use std::fs::File;
use std::path::Path;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use itertools::Itertools;
use rayon::prelude::*;

mod iter;
use iter::rangef;




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
            let wvlt_fn = |t| soulti(t, 0.02); // Placeholder wavelet
            let wvlt_bounds = [-3.0, 3.0];

            // Frequencies (1 to 9 kHz at interval of 10Hz)
            let frequencies: Vec<f32> = rangef(1000.0,9000.0,10.0).collect();

            // Do cwt
            let s = cwt(y, wvlt_fn, wvlt_bounds, frequencies, fs);

            // Write cwt data to a file
            let mut wtr = csv::Writer::from_path(output_file).unwrap();
            for row in s.into_iter() {
                let text_vec: Vec<String> = row.iter().map( |n| format!("{:e}",n) ).collect(); // Use sci notation
                wtr.write_record(&text_vec).unwrap();
            }
            wtr.flush().unwrap();
            

        }
        _ => panic!("read error or wrong wave type"),
    }
}

fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}

fn cwt(
    y: Vec<f32>,
    wvlt_fn: fn(f32) -> f32,
    wvlt_bounds: [f32; 2],
    frequencies: Vec<f32>,
    fs: u32,
) -> Vec<Vec<f32>> {
    println!("cwt fn called");

    let step = 1.0/(fs as f32);
    let mut s: Vec<Vec<f32>> = Vec::new();

    for f in &frequencies {
        let scale = 1.0/f;
        let t = rangef(wvlt_bounds[0]*scale, wvlt_bounds[1]*scale, step);
        let k = 1.0 / scale.sqrt();
        let wv: Vec<f32> = t.map( |t| k * wvlt_fn(t/scale) ).rev().collect();

        let row = conv_par(&y,&wv);
        s.push(row);
    }

    s
}

// do convolution the traditional way
fn conv(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {
    println!("conv called");

    let mut y: Vec<f32> = vec![0.0;sig.len()];

    for x in 0..sig.len() {
        for i in 0..fir.len() {
            if x+i >= sig.len() {
                break;
            } else {
                y[x] += fir[i] * sig[x+i];
            }
        }
    }

    y
}

// do convolution using parallelism
fn conv_par(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {
    println!("conv called");

    let mut y: Vec<f32> = vec![0.0;sig.len()];

    // for x in 0..sig.len() {
    //     for i in 0..fir.len() {
    //         if x+i >= sig.len() {
    //             break;
    //         } else {
    //             y[x] += fir[i] * sig[x+i];
    //         }
    //     }
    // }

    // (0..sig.len()).into_par_iter().map(|ind| {
    //     for i in 0..fir.len() {
    //         if ind+i >= sig.len() {
    //             break;
    //         } else {
    //             y[ind] += fir[i] * sig[ind+i];
    //         }
    //     }
    // });

    y.par_iter_mut().enumerate().for_each(|(ind, val)| {
        for i in 0..fir.len() {
            if ind+i >= sig.len() {
                break;
            } else {
                *val = *val + fir[i] * sig[ind+i];
            }
        }
    });

    y
}

// do convolution quickly using FFT
fn conv_fft(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {

    println!("conv_fast called");

    let n = sig.len() + fir.len() - 1;

    // Time domain
    let mut tsig: Vec<Complex<f32>> = sig.iter().pad_using(n, |_i| &0.0).map(|t| Complex::from(t)).collect();
    let mut tfir: Vec<Complex<f32>> = fir.iter().pad_using(n, |_i| &0.0).map(|t| Complex::from(t)).collect();

    // Frequency domain
    let mut fsig: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let mut ffir: Vec<Complex<f32>> = vec![Complex::zero(); n];

    println!("vecs prepared");

    // Do FFT
    let fft = FFTplanner::new(false).plan_fft(n);
    fft.process(&mut tsig, &mut fsig);
    fft.process(&mut tfir, &mut ffir);

    println!("fft done");

    // Elementwise multiplication
    // Note that normally I would divide fsig and ffir by sqrt(n) but that isn't necessary here
    let mut fres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    for i in 0..n {
        fres[i] = fsig[i] * ffir[i];
    }

    println!("elementwise multiplication done");

    // Do IFFT
    let mut tres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let fft = FFTplanner::new(true).plan_fft(n);
    fft.process(&mut fres, &mut tres);

    println!("ifft done");

    // Make real and return
    let result: Vec<f32> = tres.iter().map(|i| i.re).collect();
    result

}
