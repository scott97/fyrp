use super::Cwt;
use crate::cwt::wavelets::WaveletFn;
use crate::iter::rangef;
use crate::xcorr::cplx;
use crate::xcorr::real;
use rayon::prelude::*;
use rustfft::num_complex::Complex;

pub struct Simd {
    wvt: Box<dyn Send + Sync + WaveletFn>,
    wvt_bounds: [f32; 2],
    frequencies: Vec<f32>,
    step: f32,
    take: usize, // Length to save.
}

impl Simd {
    pub fn new(
        chunk_len: usize,
        max_wvt_len: usize,
        wvt: Box<dyn Send + Sync + WaveletFn>,
        wvt_bounds: [f32; 2],
        frequencies: &[f32],
        fs: u32,
    ) -> Self {
        Simd {
            wvt,
            wvt_bounds,
            frequencies: frequencies.to_vec(), // Make a copy
            step: 1.0 / (fs as f32),
            take: chunk_len - max_wvt_len,
        }
    }
}

impl<I: Iterator<Item = f32>> Cwt<I> for Simd {
    fn process_real(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        panic!("Not implemented");
    }

    fn process_real_par(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        panic!("Not implemented");
    }

    fn process_cplx(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        let sig_cpx: Vec<Complex<f32>> = sig.map(Complex::from).collect();

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

                let wvt: Vec<Complex<f32>> = t.map(|t| k * self.wvt.cplx(t / scale)).collect();

                cplx::xcorr_simd(&sig_cpx, &wvt)
                    .iter()
                    .take(self.take)
                    .map(|i| i.norm())
                    .collect()
            })
            .collect()
    }
    
    fn process_cplx_par(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        let sig_cpx: Vec<Complex<f32>> = sig.map(Complex::from).collect();

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

                let wvt: Vec<Complex<f32>> = t.map(|t| k * self.wvt.cplx(t / scale)).collect();

                cplx::xcorr_simd(&sig_cpx, &wvt)
                    .iter()
                    .take(self.take)
                    .map(|i| i.norm())
                    .collect()
            })
            .collect()
    }
}
