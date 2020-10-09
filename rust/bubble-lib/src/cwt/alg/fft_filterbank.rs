use super::Cwt;

use crate::cwt::wavelets::WaveletFn;
use rayon::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;
use rustfft::FFT;

pub struct FftFilterBank {
    filter_bank: Vec<Vec<Complex<f32>>>, // Calculated ahead of time and reused.
    take: usize,                         // Length to save.
}

impl FftFilterBank {
    pub fn new(
        chunk_len: usize,
        max_wvt_len: usize, // Length to discard.
        wvt: Box<dyn Send + Sync + WaveletFn>,
        frequencies: &[f32],
        fs: u32,
    ) -> FftFilterBank {
        let filter_bank: Vec<Vec<Complex<f32>>> = frequencies
            .par_iter()
            .map(|f| {
                let t = (0..chunk_len).map(|x| x as f32 / fs as f32);
                let mut wvt_t: Vec<Complex<f32>> =
                    t.map(|t| f.sqrt() * wvt.cplx(t * f)).rev().collect();

                let mut wvt_f: Vec<Complex<f32>> = vec![Complex::zero(); chunk_len];
                let fft = FFTplanner::<f32>::new(false).plan_fft(chunk_len);
                fft.process(&mut wvt_t, &mut wvt_f);

                wvt_f
            })
            .collect();

        FftFilterBank {
            filter_bank,
            take: chunk_len - max_wvt_len,
        }
    }
}

impl<I: Iterator<Item = f32>> Cwt<I> for FftFilterBank {
    fn process_real(&mut self, _sig: &mut I) -> Vec<Vec<f32>> {
        panic!("--cwt Fft does not support real wavelets. There is no performance 
                improvement with this combination, and therefore no reason to use it.");
    }
    fn process_real_par(&mut self, _sig: &mut I) -> Vec<Vec<f32>> {
        panic!("--cwt Fft does not support real wavelets. There is no performance 
                improvement with this combination, and therefore no reason to use it.");
    }


    fn process_cplx(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        // Copy signal into a vector of complex numbers.
        let mut sig_t: Vec<Complex<f32>> = sig.map(Complex::from).collect();
        // Signal length.
        let n = sig_t.len();
        let n_recip = (n as f32).recip();

        // Apply Fourier Transform to signal.
        let mut sig_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
        let fft = FFTplanner::<f32>::new(false).plan_fft(n);
        fft.process(&mut sig_t, &mut sig_f);

        // Convolution of signal with filters.
        self.filter_bank
            .iter()
            .map(|wvt| {
                // Do convolution via element-wise multiplication.
                let mut row_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                for i in 0..n {
                    row_f[i] = sig_f[i] * wvt[i] * n_recip;
                }

                // Do IFFT
                let mut row_t: Vec<Complex<f32>> = vec![Complex::zero(); n];
                let ifft = FFTplanner::<f32>::new(true).plan_fft(n);
                ifft.process(&mut row_f, &mut row_t);

                // Only take the values to save
                // Find absolute value of complex values
                row_t.iter().take(self.take).map(|i| i.norm()).collect()
            })
            .collect()
    }
    fn process_cplx_par(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        // Copy signal into a vector of complex numbers.
        let mut sig_t: Vec<Complex<f32>> = sig.map(Complex::from).collect();
        // Signal length.
        let n = sig_t.len();
        let n_recip = (n as f32).recip();

        // Apply Fourier Transform to signal.
        let mut sig_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
        let fft = FFTplanner::<f32>::new(false).plan_fft(n);
        fft.process(&mut sig_t, &mut sig_f);

        // Convolution of signal with filters.
        self.filter_bank
            .par_iter()
            .map(|wvt| {
                // Do convolution via element-wise multiplication.
                let mut row_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                for i in 0..n {
                    row_f[i] = sig_f[i] * wvt[i] * n_recip;
                }

                // Do IFFT
                let mut row_t: Vec<Complex<f32>> = vec![Complex::zero(); n];
                let ifft = FFTplanner::<f32>::new(true).plan_fft(n);
                ifft.process(&mut row_f, &mut row_t);

                // Only take the values to save
                // Find absolute value of complex values
                row_t.iter().take(self.take).map(|i| i.norm()).collect()
            })
            .collect()
    }
}

// Unit tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::cwt::*;
//     use crate::iter;
//     use assert_approx_eq::assert_approx_eq;

//     #[test]
//     fn test_cwt_fft_cpx_filterbank() {
//         let d: Vec<_> = (0..8).map(|n| (n as f32).sin()).collect();
//         let fs: u32 = 44100;

//         let chunk_len: usize = 8;
//         let peek_len: usize = 2;

//         let wvt = box wavelets::Laplace::new(0.3);
//         let frequencies: Vec<_> = iter::rangef(1e3, 2e3, 500e0).collect();

//         let mut alg = FftFilterBank::new(chunk_len, peek_len, wvt, &frequencies, fs);

//         let expected = vec![
//             vec![
//                 11.720512, 27.309483, 35.759567, 39.534855, 47.096947, 43.889164,
//             ],
//             vec![
//                 18.776264, 41.308903, 58.713436, 51.981052, 55.199570, 62.125590,
//             ],
//             vec![
//                 27.564340, 54.117737, 80.081700, 65.083590, 60.965378, 77.922485,
//             ],
//         ];

//         let actual = alg.process_par(&mut d.into_iter());

//         // Assert equal
//         assert_eq!(expected.len(), actual.len());
//         for (exp, act) in expected.iter().zip(actual.iter()) {
//             assert_eq!(exp.len(), act.len());
//             for (e, a) in exp.iter().zip(act.iter()) {
//                 assert_approx_eq!(e, a, 1e-3);
//             }
//         }
//     }
// }
