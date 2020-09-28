use crate::*;
use crate::mean_shift_clustering::MeanShiftClustering;
use crate::mean_shift_clustering::Point;
use std::f32::consts::TAU;

use crate::cwt::alg;
use crate::cwt::alg::Cwt;
use crate::cwt::wavelets;
use crate::cwt::wavelets::WaveletFn;
use crate::iter;
use crate::config;

pub struct BubbleIdentifier {
    cwt: Box<dyn Cwt<std::vec::IntoIter<f32>>>,
    parallel: bool,
    cluster_alg: Option<MeanShiftClustering>,
    threshold: f32,
    frequencies: Vec<f32>,
    fs: u32,
}

impl BubbleIdentifier {
    pub fn new(opt: &config::Opts, fs: u32) -> Self {
        const PEAK_FINDING_OVERLAP: usize = 2;
        let take = (opt.segment_size * 1e-3 * fs as f32) as usize;
        let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;
        let len = take + peek;

        let frequencies: Vec<_> = iter::rangef(
            opt.min_radius * 1e-3,
            opt.max_radius * 1e-3,
            opt.radius_resolution * 1e-3,
        )
        .map(to_freq)
        .collect();

        if opt.debug {
            println!("Lowest frequency: {}", frequencies.first().unwrap());
            println!("Highest frequency: {}", frequencies.last().unwrap());
            println!("Number of frequency bands: {}", frequencies.len());
        }

        let wvt = match opt.wavelet {
            config::Wavelet::Soulti => WaveletFn::Soulti(wavelets::Soulti::new(opt.zeta)),
            config::Wavelet::Morlet => WaveletFn::Morlet(wavelets::Morlet::new()),
        };
        let cwt: Box<dyn Cwt<std::vec::IntoIter<f32>>> = match opt.cwt {
            config::CwtAlg::FftCpxFilterBank => {
                box alg::FftCpxFilterBank::new(len, peek, wvt, &frequencies, fs)
            }
            config::CwtAlg::FftCpx => box alg::FftCpx::new(wvt, [0., 50.], &frequencies, fs),
            config::CwtAlg::Fft => {
                box alg::Fft::new(|t| wavelets::soulti(t, 0.02), [0., 50.], &frequencies, fs)
            }
            config::CwtAlg::Standard => {
                box alg::Standard::new(|t| wavelets::soulti(t, 0.02), [0., 50.], &frequencies, fs)
            }
            config::CwtAlg::Simd => {
                box alg::Simd::new(|t| wavelets::soulti(t, 0.02), [0., 50.], &frequencies, fs)
            }
        };

        BubbleIdentifier {
            cwt,
            parallel: opt.parallel,
            cluster_alg: if opt.clustering {Some(MeanShiftClustering::new(&opt))} else {None},
            threshold: opt.threshold,
            frequencies,
            fs,
        }
    }

    pub fn process(&mut self, chunk: Vec<f32>) -> Vec<(f32, f32)> {
        let mut s = if self.parallel {
            self.cwt.process_par(&mut chunk.into_iter())
        } else {
            self.cwt.process(&mut chunk.into_iter())
        };
        threshold(&mut s, self.threshold);
        self.find_bubbles(&s)
    }

    fn find_bubbles(&self, s: &[Vec<f32>]) -> Vec<(f32, f32)> {
        let fs = self.fs as f32;

        let mut peaks: Vec<Point> = Vec::new();
        for row in 1..s.len() - 1 {
            for col in 1..s[0].len() - 1 {
                // Check it is a local maximum.
                if s[row][col] > s[row + 1][col]
                    && s[row][col] > s[row - 1][col]
                    && s[row][col] > s[row][col + 1]
                    && s[row][col] > s[row][col - 1]
                {
                    let freq = self.frequencies[row] * 1e-3; // kHz.
                    let time = (col as f32) / fs * 1e3; // ms.
                    let value = s[row][col];
                    let p = Point {
                        position: (freq, time),
                        value,
                    };
                    peaks.push(p);
                }
            }
        }

        let points = if self.cluster_alg.is_some() {
            self.cluster_alg.as_ref().unwrap().cluster(&peaks)
        } else {
            peaks
        };

        points
            .into_iter()
            .map(|p| (to_radius(p.position.0), p.position.1))
            .collect()
    }
}

pub fn threshold(s: &mut Vec<Vec<f32>>, min: f32) {
    for row in s.iter_mut() {
        for val in row {
            if *val < min {
                *val = 0.;
            }
        }
    }
}

pub fn to_radius(freq: f32) -> f32 {
    (3f32 * 1.4f32 * 101.325f32).sqrt() / (freq * TAU)
}

pub fn to_freq(radius: f32) -> f32 {
    (3f32 * 1.4f32 * 101.325f32).sqrt() / (radius * TAU)
}
