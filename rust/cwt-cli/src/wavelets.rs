use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

use faster::*;
use itertools::Itertools;
use packed_simd::*;
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

            conv_simd2(&y, &wv)[wv.len()..].to_vec()
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

// do convolution using SIMD - note this lib is complicated, lets not use it
fn conv_simd(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
    // dbg!("conv_simd");

    let lx = x.len();
    let lh = h.len();
    let lxh = lx + lh - 1;

    let mut x = x.to_vec(); // doing the copying probably is slowing it down
    x.extend(vec![0.0f32; lh - 1].into_iter());
    let mut y = vec![0.0; lxh];

    #[cfg(not(any(
        target_feature = "avx2",
        target_feature = "avx",
        target_feature = "sse",
        target_feature = "avx512"
    )))]
    {
        dbg!("SIMD not supported?");
    }

    // doesn't crash, but does it work and is it faster?
    // also i think this might be the wrong length
    for m in 0..lh {
        y = (
            (&y[0..lx]).simd_iter(f32s(0.0)),
            (&x[m..(m + lx)]).simd_iter(f32s(0.0)),
        )
            .zip()
            .simd_map(|(y_p, x_s)| y_p + x_s * f32s(h[m]))
            .scalar_collect();
    }

    y
}

// do convolution using SIMD (alternative library)
// fn conv_simd2(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
//     // dbg!("conv_simd");

//     let lx = x.len();
//     let lxch = lx - (lx % 16); // Length of x rounded down to nearest 16.
//     let lh = h.len();
//     let lxh = lx + lh - 1;

//     let mut y = vec![0.0; lxh]; // It might be worth trying extending to the next 16, and padding with zeros

//     for m in 0..lh {
//         // TODO - handle left and right zeros
//         // TODO - shift x

//         dbg!(h[m]);

//         // Process chunks
//         dbg!("process chunks");
//         for ch in (0..lxch).step_by(16) {
//             let x_chunk = f32x16::from_slice_unaligned(&x[ch..]);
//             let y_chunk = f32x16::from_slice_unaligned(&y[ch..]);
//             let result = y_chunk + x_chunk * f32x16::splat(h[m]);
//             result.write_to_slice_unaligned(&mut y[ch..]);
//         }
//         // Remaining elements
//         dbg!("single elements");
//         for i in (lxch..lx) {
//             y[i] += x[i] * h[m];
//         }
//         dbg!(&y);
//     }

//     y
// }

// /// works on chunks, pads to get up to a chunk size
// /// issue: shifts x in wrong direction
// fn conv_simd2(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
//     dbg!("conv_simd");

//     let lx = x.len();
//     let lh = h.len();
//     let lxh = lx + lh - 1;
//     let lxch = lx - (lx % 16) + 16; 
//     let lxhch = lxch + lh;

//     let mut y = vec![0.0; lxhch];
//     let mut xm = x.to_vec();
//     xm.resize(lxhch, 0.);
    
//     dbg!(&xm); // initial values of extended x

//     for m in 0..lh {
//         // TODO - fix bugs
//         dbg!(h[m]);
//         for ch in (0..lxch).step_by(16) {
//             dbg!(&ch);
//             let x_chunk = f32x16::from_slice_unaligned(&xm[(ch+m)..(ch+m+16)]); // Issue, shifts wrong direction. This isn't as simple as changing the sign, I also need to consider the edge of the backing array.
//             let y_chunk = f32x16::from_slice_unaligned(&y[(ch)..(ch+16)]);
//             let result = y_chunk + x_chunk * f32x16::splat(h[m]);
//             result.write_to_slice_unaligned(&mut y[ch..]);
//         }
//         dbg!(&y);
//     }

//     //y[0..lxh].to_vec()
//     y.truncate(lxh);
//     y
// }


/// correct implementation
fn conv_simd2(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
    dbg!("conv_simd");

    let lx = x.len();
    let lh = h.len();
    let lxh = lx + lh - 1;
    let lxch = lx - (lx % 16) + 16; 
    let lxhch = lxch + lh;

    let mut y = vec![0.0; lxhch];

    let mut xm = vec![0.; lh];
    xm.extend(x.iter()); // pad left w/ zeros so shifts of x don't read outside array.
    xm.resize(lxhch, 0.); // pad right w/ zeros to chunk size.
    
    dbg!(&xm); // initial values of extended x

    for m in 0..lh {
        dbg!(h[m]);
        for ch in (0..lxch).step_by(16) {
            dbg!(&ch);
            let x_chunk = f32x16::from_slice_unaligned(&xm[(ch+lh-m)..(ch+lh-m+16)]);
            let y_chunk = f32x16::from_slice_unaligned(&y[(ch)..(ch+16)]);
            let result = y_chunk + x_chunk * f32x16::splat(h[m]);
            result.write_to_slice_unaligned(&mut y[ch..]);
        }
        dbg!(&y);
    }

    //y[0..lxh].to_vec()
    y.truncate(lxh);
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
    // Dividing each individually by sqrt(n) is the same as dividing both by n.
    let mut fres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let n_inv = 1. / (n as f32);
    for i in 0..n {
        fres[i] = fsig[i] * ffir[i] * n_inv;
    }

    // Do IFFT
    let mut tres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let fft = FFTplanner::new(true).plan_fft(n);
    fft.process(&mut fres, &mut tres);

    // Make real and return
    let result: Vec<f32> = tres.iter().map(|i| i.re).collect();
    result
}

// do convolution using parallelism !!!not correct
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
