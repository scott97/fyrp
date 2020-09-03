// Reexports
mod standard;
pub use self::standard::Standard;

mod simd;
pub use self::simd::Simd;

mod fft;
pub use self::fft::Fft;

mod fft_cpx;
pub use self::fft_cpx::FftCpx;

mod fft_cpx_filterbank;
pub use self::fft_cpx_filterbank::FftCpxFilterBank;

// Traits
pub trait Cwt {
    fn process(&mut self, sig: &mut impl Iterator<Item = f32>) -> Vec<Vec<f32>>;
    fn process_par(&mut self, sig: &mut impl Iterator<Item = f32>) -> Vec<Vec<f32>>;
}

// Benchmarking tests
#[cfg(test)]
mod tests {
    use crate::cwt::alg;
    use crate::cwt::alg::Cwt;
    use crate::cwt::wavelets;
    use crate::iter;
    use crate::get_data;
    
    
    use test::Bencher;


    // All tests are on 100 ms of audio, with frequency bands
    // from 1 kHz to 9 kHz with an interval of 20Hz.
    const N: usize = 4410; // 100 ms × 44100 Hz = 4410 samples.
    const MAX_WVT_LEN: usize  = 2205; // Length at 1 kHz

    #[bench]
    fn bench_standard(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Standard::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_standard(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Standard::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_simd(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Simd::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_simd(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Simd::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Fft::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::Fft::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft_cpx(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::FftCpx::new(|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft_cpx(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::FftCpx::new(|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft_cpx_filter_bank(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::FftCpxFilterBank::new(N,MAX_WVT_LEN,|t| wavelets::soulti_cpx(t, 0.02), &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft_cpx_filter_bank(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data() {
            let mut y = d.into_iter().take(N);
            let mut cwt = alg::FftCpxFilterBank::new(N,MAX_WVT_LEN,|t| wavelets::soulti_cpx(t, 0.02), &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }


}
