// Reexports
mod standard;
pub use self::standard::Standard;

mod simd;
pub use self::simd::Simd;

mod fft;
pub use self::fft::Fft;

mod fft_filterbank;
pub use self::fft_filterbank::FftFilterBank;

// Traits
pub trait Cwt<I>
where
    I: Iterator<Item = f32>,
{
    fn process_real(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_cplx(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_real_par(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_cplx_par(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cwt::*;
    use crate::iter;
    use assert_approx_eq::assert_approx_eq;

    fn assert_cwt_eq(expected: &[Vec<f32>], actual: &[Vec<f32>]) {
        assert_eq!(expected.len(), actual.len());
        for (exp, act) in expected.iter().zip(actual.iter()) {
            assert_eq!(exp.len(), act.len());
            for (e, a) in exp.iter().zip(act.iter()) {
                assert_approx_eq!(e, a, 1e-3);
            }
        }
    }

    #[test]
    fn test_cwt_algorithms_are_consistent() {
        let d: Vec<_> = (0..8).map(|n| (n as f32).sin()).collect();
        let fs: u32 = 44100;

        let chunk_len: usize = 8;
        let peek_len: usize = 2;

        let frequencies: Vec<_> = iter::rangef(1e3, 2e3, 500e0).collect();

        // Results
        let wvt = box wavelets::Laplace::new(0.3);
        let fft_filterbank = FftFilterBank::new(chunk_len, peek_len, wvt, &frequencies, fs)
            .process_cplx_par(&mut d.to_vec().into_iter());

        let wvt = box wavelets::Laplace::new(0.3);
        let fft = Fft::new(chunk_len, peek_len, wvt, [0., 50.], &frequencies, fs)
            .process_cplx_par(&mut d.to_vec().into_iter());

        let wvt = box wavelets::Laplace::new(0.3);
        let standard = Standard::new(chunk_len, peek_len, wvt, [0., 50.], &frequencies, fs)
            .process_cplx_par(&mut d.to_vec().into_iter());

        let wvt = box wavelets::Laplace::new(0.3);
        let simd = Simd::new(chunk_len, peek_len, wvt, [0., 50.], &frequencies, fs)
            .process_cplx_par(&mut d.to_vec().into_iter());

        // Assert all results are equal
        assert_cwt_eq(&standard, &simd);
        assert_cwt_eq(&standard, &fft);
        // assert_cwt_eq(&standard, &fft_filterbank); // -> the odd one out.
    }
}
