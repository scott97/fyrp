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
    use std::fs::File;
    use std::path::Path;
    use test::Bencher;

    fn get_data(duration: f32) -> Option<(Vec<f32>, u32)> {
        let input_file = Path::new("data.wav");
        let mut inp_file = File::open(input_file).unwrap();
        let (header, data) = wav::read(&mut inp_file).unwrap();
        let fs = header.sampling_rate;

        // Remap to range -1.0 to 1.0, and take only 1000ms
        if let wav::BitDepth::Sixteen(raw_signal) = data {
            let y = raw_signal
                .iter()
                .map(|x| (*x as f32) / (i16::MAX as f32))
                .take((duration * fs as f32) as usize)
                .collect();
            Some((y, fs))
        } else {
            None
        }
    }

    // All tests are on 100 ms of audio, with frequency bands
    // from 1 kHz to 9 kHz with an interval of 20Hz.

    #[bench]
    fn bench_standard(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Standard::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_standard(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Standard::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_simd(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Simd::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_simd(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Simd::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Fft::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::Fft::new(|t| wavelets::soulti(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft_cpx(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::FftCpx::new(|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft_cpx(b: &mut Bencher) {
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(0.100) {
            let mut y = d.into_iter();
            let mut cwt = alg::FftCpx::new(|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }

    #[bench]
    fn bench_fft_cpx_filter_bank(b: &mut Bencher) {
        let dur = 0.100;
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(dur) {
            let mut y = d.into_iter();
            let mut cwt = alg::FftCpxFilterBank::new(dur,|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process(&mut y));
        } 
    }

    #[bench]
    fn bench_par_fft_cpx_filter_bank(b: &mut Bencher) {
        let dur = 0.100;
        let f: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        if let Some((d, fs)) = get_data(dur) {
            let mut y = d.into_iter();
            let mut cwt = alg::FftCpxFilterBank::new(dur,|t| wavelets::soulti_cpx(t, 0.02), [0.0, 50.0], &f, fs);
            b.iter(|| cwt.process_par(&mut y));
        } 
    }


}
