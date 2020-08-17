use super::Cwt;
use rustfft::num_complex::Complex;
use rayon::prelude::*;
use crate::conv;
use crate::iter::rangef;

pub struct FftCpx {
    wvt_fn: fn(f32) -> Complex<f32>,
    wvt_bounds: [f32; 2],
    frequencies: Vec<f32>,
    step: f32,
}

impl FftCpx {
    pub fn new(
        wvt_fn: fn(f32) -> Complex<f32>,
        wvt_bounds: [f32; 2],
        frequencies: &Vec<f32>,
        fs: u32,
    ) -> FftCpx {
        FftCpx {
            wvt_fn: wvt_fn,
            wvt_bounds: wvt_bounds,
            frequencies: frequencies.to_vec(), // Make a copy
            step: 1.0 / (fs as f32),
        }
    }
}

impl Cwt for FftCpx {
    #[exec_time]
    fn process(&mut self, sig: &Vec<f32>) -> Vec<Vec<f32>> {
        self.frequencies
            .iter()
            .map(|f| {
                let scale = 1.0 / f;
                let t = rangef(
                    self.wvt_bounds[0] * scale,
                    self.wvt_bounds[1] * scale,
                    self.step,
                );
                let k = 1.0 / scale.sqrt();
                let wvt: Vec<Complex<f32>> = t.map(|t| k * (self.wvt_fn)(t / scale)).rev().collect();

                conv::conv_fft_cpx(&sig, &wvt)[wvt.len()..].to_vec()
            })
            .collect()
    }
    #[exec_time]
    fn process_par(&mut self, sig: &Vec<f32>) -> Vec<Vec<f32>> {
        self.frequencies
            .par_iter()
            .map(|f| {
                let scale = 1.0 / f;
                let t = rangef(
                    self.wvt_bounds[0] * scale,
                    self.wvt_bounds[1] * scale,
                    self.step,
                );
                let k = 1.0 / scale.sqrt();
                let wvt: Vec<Complex<f32>> = t.map(|t| k * (self.wvt_fn)(t / scale)).rev().collect();

                conv::conv_fft_cpx(&sig, &wvt)[wvt.len()..].to_vec()
            })
            .collect()
    }
}
