use std::fs::File;
use std::path::Path;

fn main() {
    println!("CWT implementation that can be put on a microcontroller");

    // Get signal y(t)
    let mut inp_file = File::open(Path::new("./data.wav")).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    let fs = header.sampling_rate;

    match data {
        wav::BitDepth::Sixteen(raw_signal) => {
            println!("read success (i16), length {:?}", raw_signal.len());

            // Remap to range -1.0 to 1.0
            let y = raw_signal
                .iter()
                .map(|x| (*x as f32) / (i16::MAX as f32))
                .collect::<Vec<f32>>();

            // Wavelet function
            let wvlt_fn = |t| soulti(t, 0.02); // Placeholder wavelet
            let wvlt_bounds = [-3.0, 3.0];

            // Frequencies (1 to 9 kHz at interval of 10Hz)
            let frequencies: Vec<f32> = IterStep::new(1000.0,9000.0,10.0).collect();

            // Do cwt
            let s = cwt(y, wvlt_fn, wvlt_bounds, frequencies, fs);
        }
        _ => panic!("read error or wrong wave type"),
    }
}

fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}

fn cwt(
    y: Vec<f32>,
    wvlt_fn: fn(f32) -> f32,
    wvlt_bounds: [f32; 2],
    frequencies: Vec<f32>,
    fs: u32,
) -> Vec<Vec<f32>> {
    println!("cwt fn called");

    let step = 1.0/(fs as f32);

    for f in &frequencies {
        let scale = 1.0/f;
        let t = IterStep::new(wvlt_bounds[0]*scale, wvlt_bounds[1]*scale, step);
        let wv: Vec<f32> = t.map(|t| wvlt_fn(t/scale)).rev().collect();

        // row = conv(y,fliplr(wv)) .* (1/sqrt(scale));
        // s(i,:) = row(length(wv):end);
    }

    vec![vec![]] // Placeholder empty data
}



struct IterStep {
    value: Option<f32>,
    start: f32,
    end: f32,
    step: f32,
}

impl IterStep {
    fn new(start: f32, end: f32, step: f32) -> IterStep {
        IterStep { value: None, start, end, step }
    }
}

impl Iterator for IterStep {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Some(v) => { 
                if v < self.end {
                    self.value = Some(v + self.step);
                    self.value
                } else {
                    None
                }
            },
            None => {
                self.value = Some(self.start);
                self.value
            },
        }
    }
}
impl DoubleEndedIterator for IterStep {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.value {
            Some(v) => { 
                if v > self.start {
                    self.value = Some(v - self.step);
                    self.value
                } else {
                    None
                }
            },
            None => {
                self.value = Some(self.end);
                self.value
            },
        }
    }
}

    // // Test iterator
    // for i in IterStep::new(0.0,5.0,0.5) {
    //     println!("i: {:?}",i);
    // }
    // for i in IterStep::new(0.0,5.0,0.5).rev() {
    //     println!("i: {:?}",i);
    // }