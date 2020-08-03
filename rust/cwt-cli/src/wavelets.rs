use rayon::prelude::*;
use super::iter::rangef;
use super::conv;

// Wavelets
pub fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}

// Various implementations of CWT
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

            conv::conv(&y, &wv)[wv.len()..].to_vec()
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

            conv::conv(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

#[exec_time]
pub fn cwt_simd(
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

            conv::conv_simd(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

#[exec_time]
pub fn cwt_par_simd(
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

            conv::conv_simd(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

#[exec_time]
pub fn cwt_fft(
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

            conv::conv_fft(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}

#[exec_time]
pub fn cwt_par_fft(
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

            conv::conv_fft(&y, &wv)[wv.len()..].to_vec()
        })
        .collect()
}
