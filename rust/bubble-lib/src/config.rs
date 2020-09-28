// use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum CwtAlg {
        Fft,
        FftFilterBank,
        Simd,
        Standard
    }
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum Wavelet {
        Soulti,
        Morlet,
    }
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum ClusteringWindow {
        Circular,
        Ellipse,
        Gaussian,
        GaussianEllipse
    }
}


#[derive(StructOpt, Debug, Clone)]
pub struct Opts {
    /// Continuous wavelet transform algorithm
    #[structopt(short,long,possible_values = &CwtAlg::variants(), case_insensitive = true, default_value = "FftFilterBank")]
    pub cwt: CwtAlg,

    /// Activate debug mode.
    #[structopt(short, long)]
    pub debug: bool,

    /// Segment size (ms)
    #[structopt(short, long, default_value = "200")]
    pub segment_size: f32,

    /// Threshold value (unitless)
    #[structopt(short, long, default_value = "100")]
    pub threshold: f32,

    /// Radius resolution (mm)
    #[structopt(short, long, default_value = "0.02")]
    pub radius_resolution: f32,

    /// Minimum radius (mm)
    #[structopt(short = "m", long, default_value = "0.30")]
    pub min_radius: f32,

    /// Maximum radius (mm)
    #[structopt(short = "M", long, default_value = "3.00")]
    pub max_radius: f32,

    /// Disable parallel processing of data
    #[structopt(long = "no-parallel", parse(from_flag = std::ops::Not::not))]
    pub parallel: bool,

    /// Disable clustering of close data points
    #[structopt(long = "no-clustering", parse(from_flag = std::ops::Not::not))]
    pub clustering: bool,

    /// Clustering Window
    #[structopt(long="window",possible_values = &ClusteringWindow::variants(), case_insensitive = true, default_value = "Circular")]
    pub clustering_window: ClusteringWindow,

    /// Clustering Window Bandwidth (For circular windows, specify a single number, for ellipse windows, specify two)
    #[structopt(long = "bandwidth", default_value = "15")]
    pub clustering_window_bandwidths: Vec<f32>,

    /// Clustering Window Maximum Iterations
    /// Choose a value which is sufficient for clustering to work.
    /// Higher values give slower performance.
    #[structopt(short = "i", long, default_value = "20")]
    pub max_iterations: u32,

    /// Wavelet function
    #[structopt(short,long,possible_values = &Wavelet::variants(), case_insensitive = true, default_value = "Soulti")]
    pub wavelet: Wavelet,

    /// Zeta is used by the soulti wavelet function
    #[structopt(short, long, default_value = "0.02", required_if("wavelet", "Soulti"))]
    pub zeta: f32,
}
