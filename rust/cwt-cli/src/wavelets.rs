use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

use itertools::Itertools;
use rayon::prelude::*;

use super::iter::rangef;

pub fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}

#[exec_time]
pub fn cwt(
    y: &Vec<f32>,
    wvlt_fn: fn(f32) -> f32,
    wvlt_bounds: [f32; 2],
    frequencies: &Vec<f32>,
    fs: u32,
) -> Vec<Vec<f32>> {
    let step = 1.0 / (fs as f32);

    frequencies
        .iter()
        .map(|f| {
            let scale = 1.0 / f;
            let t = rangef(wvlt_bounds[0] * scale, wvlt_bounds[1] * scale, step);
            let k = 1.0 / scale.sqrt();
            let wv: Vec<f32> = t.map(|t| k * wvlt_fn(t / scale)).collect();

            conv(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

#[exec_time]
pub fn cwt_par(
    y: &Vec<f32>,
    wvlt_fn: fn(f32) -> f32,
    wvlt_bounds: [f32; 2],
    frequencies: &Vec<f32>,
    fs: u32,
) -> Vec<Vec<f32>> {
    let step = 1.0 / (fs as f32);
    
    frequencies
        .par_iter()
        .map(|f| {
            let scale = 1.0 / f;
            let t = rangef(wvlt_bounds[0] * scale, wvlt_bounds[1] * scale, step);
            let k = 1.0 / scale.sqrt();
            let wv: Vec<f32> = t.map(|t| k * wvlt_fn(t / scale)).rev().collect();

            conv(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

// do convolution the normal way
fn conv(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
    //dbg!("conv");

    let n = x.len() + h.len() - 1;
    let mut y: Vec<f32> = vec![0.0; n];

    for i in 0..n {
        for j in 0..h.len() {
            if let Some(d) = i.checked_sub(j) {
                y[i] += x.get(d).unwrap_or(&0.0) * h[j];
            }
        }
    }

    y
}

// do convolution using FFT
fn conv_fft(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {
    //dbg!("conv_fft");
    
    let n = sig.len() + fir.len() - 1;

    // Time domain
    let mut tsig: Vec<Complex<f32>> = sig
        .iter()
        .pad_using(n, |_i| &0.0)
        .map(|t| Complex::from(t))
        .collect();
    let mut tfir: Vec<Complex<f32>> = fir
        .iter()
        .pad_using(n, |_i| &0.0)
        .map(|t| Complex::from(t))
        .collect();

    // Frequency domain
    let mut fsig: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let mut ffir: Vec<Complex<f32>> = vec![Complex::zero(); n];

    // Do FFT
    let fft = FFTplanner::new(false).plan_fft(n);
    fft.process(&mut tsig, &mut fsig);
    fft.process(&mut tfir, &mut ffir);

    // Elementwise multiplication
    // Note that normally I would divide fsig and ffir by sqrt(n) but that isn't necessary here
    let mut fres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    for i in 0..n {
        fres[i] = fsig[i] * ffir[i];
    }

    // Do IFFT
    let mut tres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let fft = FFTplanner::new(true).plan_fft(n);
    fft.process(&mut fres, &mut tres);

    // Make real and return
    let result: Vec<f32> = tres.iter().map(|i| i.re).collect();
    result
}

// do convolution using parallelism
// fn conv_par(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {
//     let mut y: Vec<f32> = vec![0.0; fir.len() + sig.len() - 1];

//     y.par_iter_mut().enumerate().for_each(|(ind, val)| {
//         for i in 0..fir.len() {
//             if ind + i >= sig.len() {
//                 break;
//             } else {
//                 *val = *val + fir[i] * sig[ind + i];
//             }
//         }
//     });

//     y
// }
