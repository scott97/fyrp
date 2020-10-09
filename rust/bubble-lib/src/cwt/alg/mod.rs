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
pub trait Cwt<I> where I: Iterator<Item = f32> {
    fn process_real(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_cplx(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_real_par(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
    fn process_cplx_par(&mut self, sig: &mut I) -> Vec<Vec<f32>>;
}
