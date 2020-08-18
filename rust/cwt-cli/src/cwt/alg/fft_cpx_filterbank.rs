use super::Cwt;
use crate::iter::rangef;
use itertools::Itertools;
use rayon::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

pub struct FftCpxFilterBank {
    filter_bank: Vec<Vec<Complex<f32>>>, // calculated at new
}

impl FftCpxFilterBank {
    pub fn new(
        signal_duration: f32,
        wvt_fn: fn(f32) -> Complex<f32>,
        wvt_bounds: [f32; 2],
        frequencies: &Vec<f32>,
        fs: u32,
    ) -> FftCpxFilterBank {
        let sig_len = (signal_duration * fs as f32).ceil() as usize;

        let step = 1.0 / (fs as f32);
        let filter_bank: Vec<Vec<Complex<f32>>> = frequencies
            .par_iter()
            .map(|f| {
                let scale = 1.0 / f;
                let t = rangef(wvt_bounds[0] * scale, wvt_bounds[1] * scale, step);
                let k = 1.0 / scale.sqrt();
                let zeros = vec![Complex::zero(); sig_len - 1].into_iter();
                let mut wvt_t: Vec<Complex<f32>> = t
                    .map(|t| k * wvt_fn(t / scale))
                    .chain(zeros)
                    .rev()
                    .collect();
                let n = wvt_t.len();
                let mut wvt_f: Vec<Complex<f32>> = vec![Complex::zero(); n];

                let fft = FFTplanner::new(false).plan_fft(n);
                fft.process(&mut wvt_t, &mut wvt_f);

                wvt_f
            })
            .collect();

        FftCpxFilterBank {
            filter_bank: filter_bank,
        }
    }
}

impl Cwt for FftCpxFilterBank {
    fn process(&mut self, sig: &mut impl Iterator<Item = f32>) -> Vec<Vec<f32>> {
        let sig: Vec<f32> = sig.collect();
        // Convolution of signal with filters
        self.filter_bank
            .iter()
            .map(|wvt| {
                // Initial setup.
                let n = wvt.len();
                let fft = FFTplanner::new(false).plan_fft(n);
                let ifft = FFTplanner::new(true).plan_fft(n);

                // Apply Fourier Transform to signal.
                let mut sig_t: Vec<Complex<f32>> = sig
                    .iter()
                    .pad_using(n, |_i| &0.0)
                    .map(|t| Complex::from(t))
                    .collect();
                let mut sig_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                fft.process(&mut sig_t, &mut sig_f);

                // Do convolution via element-wise multiplication.
                let mut row_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                let n_inv = 1. / (n as f32);
                for i in 0..n {
                    row_f[i] = sig_f[i] * wvt[i] * n_inv;
                }

                // Do IFFT
                let mut row_t: Vec<Complex<f32>> = vec![Complex::zero(); n];
                ifft.process(&mut row_f, &mut row_t);

                // Only take n values, where n is the length of the signal
                // Find absolute value of complex values
                row_t.iter().take(sig.len()).map(|i| i.norm()).collect()
            })
            .collect()
    }
    fn process_par(&mut self, sig: &mut impl Iterator<Item = f32>) -> Vec<Vec<f32>> {
        let sig: Vec<f32> = sig.collect();
        // Convolution of signal with filters
        self.filter_bank
            .par_iter()
            .map(|wvt| {
                // Initial setup.
                let n = wvt.len();
                let fft = FFTplanner::new(false).plan_fft(n);
                let ifft = FFTplanner::new(true).plan_fft(n);

                // Apply Fourier Transform to signal.
                let mut sig_t: Vec<Complex<f32>> = sig
                    .iter()
                    .pad_using(n, |_i| &0.0)
                    .map(|t| Complex::from(t))
                    .collect();
                let mut sig_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                fft.process(&mut sig_t, &mut sig_f);

                // Do convolution via element-wise multiplication.
                let mut row_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                let n_inv = 1. / (n as f32);
                for i in 0..n {
                    row_f[i] = sig_f[i] * wvt[i] * n_inv;
                }

                // Do IFFT
                let mut row_t: Vec<Complex<f32>> = vec![Complex::zero(); n];
                ifft.process(&mut row_f, &mut row_t);

                // Only take n values, where n is the length of the signal
                // Find absolute value of complex values
                row_t.iter().take(sig.len()).map(|i| i.norm()).collect()
            })
            .collect()
    }
}
