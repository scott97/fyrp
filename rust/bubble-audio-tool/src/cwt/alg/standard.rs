use super::Cwt;
use crate::conv;
use crate::iter::rangef;
use rayon::prelude::*;

pub struct Standard {
    wvt_fn: fn(f32) -> f32,
    wvt_bounds: [f32; 2],
    frequencies: Vec<f32>,
    step: f32,
}

impl Standard {
    pub fn new(
        wvt_fn: fn(f32) -> f32,
        wvt_bounds: [f32; 2],
        frequencies: &[f32],
        fs: u32,
    ) -> Standard {
        Standard {
            wvt_fn,
            wvt_bounds,
            frequencies: frequencies.to_vec(), // Make a copy
            step: 1.0 / (fs as f32),
        }
    }
}

impl<I: Iterator<Item = f32>> Cwt<I> for Standard {
    fn process(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        let sig: Vec<f32> = sig.collect();
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
                let wvt: Vec<f32> = t.map(|t| k * (self.wvt_fn)(t / scale)).collect();

                conv::conv(&sig, &wvt)[..sig.len()].to_vec()
            })
            .collect()
    }
    fn process_par(&mut self, sig: &mut I) -> Vec<Vec<f32>> {
        let sig: Vec<f32> = sig.collect();
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
                let wvt: Vec<f32> = t.map(|t| k * (self.wvt_fn)(t / scale)).collect();

                conv::conv(&sig, &wvt)[wvt.len()..].to_vec()
            })
            .collect()
    }
}