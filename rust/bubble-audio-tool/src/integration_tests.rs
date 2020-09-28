#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    use std::path::Path;

    // #[test]
    // fn integration_test() {
    //     let opt = config::Opt {
    //         cwt: config::CwtAlg::FftCpxFilterBank,
    //         debug: false,
    //         input: Path::new("test_data/in/data.wav").to_path_buf(),
    //         out_dir: Path::new("test_data/out").to_path_buf(),
    //         segment_size: 200., // ms
    //         threshold: 100.,
    //         radius_resolution: 0.02, // mm
    //         min_radius: 0.30,        // mm
    //         max_radius: 3.00,        // mm
    //         parallel: true,
    //         clustering: true,
    //         clustering_window: config::ClusteringWindow::Circular,
    //         clustering_window_bandwidths: vec![15.],
    //         max_iterations: 20,
    //         scaleograms: false,
    //         wavelet: config::Wavelet::Soulti,
    //         zeta: 0.02,
    //     };

    //     run(&opt);

    //     let expected =
    //         fs::read_to_string("test_data/expected/bubbles0.csv").expect("Expected file not found");
    //     let actual =
    //         fs::read_to_string("test_data/out/bubbles0.csv").expect("Result file not found");

    //     assert_eq!(expected,actual);
    // }
}
