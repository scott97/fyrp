
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

