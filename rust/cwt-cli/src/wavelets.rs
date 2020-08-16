// todo
// verify valid code
// impl apply
// test
// refactor other files to match this
// impl old cwt functions as CWT trait

pub trait Cwt {
    fn apply(&self, y: &Vec<f32>) -> Vec<Vec<f32>>;
}

pub struct CwtParallelFftFilterBank {
    sig_len: u32,
    wvt_len: u32,
    len: u32,
    filter_bank: Vec<Complex<f32>>, // calculated at new
    sig_f: Vec<Complex<f32>>,       // preallocated to length, then reused every apply
    fft: rustfft::FFTplanner,       // plan once
    ifft: rustfft::FFTplanner,       // plan once
}

impl Cwt for CwtParallelFftFilterBank {
    fn apply(&self, sig: &Vec<f32>) -> Vec<Vec<f32>> {
        // Apply Fourier Transform to Signal.
        let mut sig_t: Vec<Complex<f32>> = sig
            .iter()
            .pad_using(self.len, |_i| &0.0)
            .map(|t| Complex::from(t))
            .collect();

        self.fft.process(&mut sig_t, &mut self.sig_f);

        // Convolution of signal with filters
        filter_bank
            .par_iter()
            .map(|wvt_f| {
                // Do convolution via elementwise multiplication
                let mut res_f: Vec<Complex<f32>> = vec![Complex::zero(); self.len];
                let n_inv = 1. / (self.len as f32);
                for i in 0..n {
                    res_f[i] = sig_f[i] * wvt_f[i] * n_inv;
                }

                // Do IFFT
                let mut tres: Vec<Complex<f32>> = vec![Complex::zero(); n];
                self.ifft.process(&mut res_f, &mut tres);

                // Find absolute value
                let result: Vec<f32> = tres.iter().map(|i| i.norm()).collect();
                result[self.wvt_len..].to_vec()
            })
            .collect()
    }
    
    fn new(
        signal_duration: f32,
        wvt_bounds: [f32; 2],
        wvt_fn: fn(f32) -> Complex<f32>,
        frequencies: &Vec<f32>,
        fs: u32,
    ) -> CwtParallelFft {
        let sig_len: u32 = (signal_duration * fs as f32).ceil();
        let wvt_len: u32 = ((wvt_bounds[1] - wvt_bounds[0]) * fs as f32).ceil();
        let n = sig_len + wvt_len - 1;
        let fft = FFTplanner::new(false).plan_fft(n);
        let ifft = FFTplanner::new(true).plan_fft(n);


        let step = 1.0 / (fs as f32);
        let filter_bank = frequencies
            .par_iter()
            .map(|f| {
                let scale = 1.0 / f;
                let t = rangef(wvt_bounds[0] * scale, wvt_bounds[1] * scale, step);
                let k = 1.0 / scale.sqrt();

                let mut wvt_f: Vec<Complex<f32>> = vec![Complex::zero(); n];
                let mut wvt_t: Vec<Complex<f32>> = t.map(|t| k * wvt_fn(t / scale)).rev().collect();
                wvt_t.resize(n, Complex::zero());

                fft.process(&mut wvt_t, &mut wvt_f);

                wvt_f
            })
            .collect();

        CwtParallelFftFilterBank {
            sig_len: sig_len,
            wvt_len: wvt_len,
            len: sig_len + wvt_len -1,
            filter_bank: filter_bank,
            sig_f: vec![Complex::zero(); n],
            fft: fft,
            ifft: ifft,
        }
    }
}
