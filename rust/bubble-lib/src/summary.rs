use crate::config;

pub struct Joiner {
    segment_size: f32,
    data: Vec<(f32, f32)>,
}

impl Joiner {
    pub fn new(opt: &config::Opts) -> Joiner {
        Joiner {
            segment_size: opt.segment_size,
            data: Vec::new(),
        }
    }
    pub fn append(&mut self, idx: isize, data: &[(f32, f32)]) {
        let size = self.segment_size;
        let iter = data.iter().map(|&(r,t)| {
            (r,t+size*(idx as f32))
        });
        self.data.extend(iter)
    }
    pub fn get_joined(&self) -> Vec<(f32, f32)> {
        self.data.to_owned()
    }
}



// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use config::Opts;

    #[test]
    fn test_joiner() {
        let mut joiner = Joiner::new(&Opts {
            // Relevant to this test
            segment_size: 200., // ms

            // Not relevant to this test
            cwt: config::CwtAlg::Standard,
            wavelet_type: config::WaveletType::CplxWavelet,
            parallel: true,
            radius_resolution: 0.02, 
            debug: false,
            threshold: 100.,
            min_radius: 0.30, // mm
            max_radius: 3.00, // mm
            clustering: true,
            clustering_window: config::ClusteringWindow::Circular,
            clustering_window_bandwidths: vec![15.],
            max_iterations: 20,
            wavelet: config::Wavelet::Laplace,
            zeta: 0.02,
        });

        joiner.append(1, &[(0.1, 20.0),(0.15, 30.0),(0.2, 40.0)]);
        joiner.append(2, &[(0.1, 20.0),(0.15, 30.0),(0.2, 40.0)]);

        let expected = vec![(0.1, 220.0),(0.15, 230.0),(0.2, 240.0),(0.1, 420.0),(0.15, 430.0),(0.2, 440.0)];
        let actual = joiner.get_joined();

        assert_eq!(expected, actual);


    }
}