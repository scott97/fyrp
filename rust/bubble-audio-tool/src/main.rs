#![feature(test)] // for benchmarks.
#![feature(box_syntax)] // for box.
#![feature(trait_alias)] // for trait _ = _.

extern crate test; // for benchmarks.

#[macro_use]
extern crate approx;

mod fileio;

use bubble_lib::{analysis, config, segmenter, summary};
use segmenter::Segmenter;
use std::path::PathBuf;
use std::thread;
use structopt::clap::arg_enum;
use structopt::StructOpt;
use winapi_util::console::{Color, Console, Intense};

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "Bubble audio tool",
    about = "A tool for analysing hydrophone data to identify bubbles."
)]
pub struct CmdOpts {
    /// Input file (.wav only)
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Output directory
    #[structopt(parse(from_os_str), default_value = "./")]
    pub out_dir: PathBuf,

    /// Output file type
    #[structopt(long,possible_values = &OutputType::variants(), case_insensitive = true, default_value = "Csv")]
    pub out_type: OutputType,

    /// Export Scaleograms
    #[structopt(long)]
    pub scaleograms: bool,

    #[structopt(flatten)]
    pub opts: config::Opts,
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum OutputType {
        Csv,
        Svg,
        Both,
    }
}

fn main() {
    let cmd = CmdOpts::from_args();

    if cmd.opts.debug {
        let mut con = Console::stdout().unwrap();
        con.fg(Intense::Yes, Color::Magenta).unwrap();
        println!("Debug mode enabled.");
        con.reset().unwrap();
        println!("Configuration: {:#?}", &cmd);
    }

    run(&cmd);
}

pub fn run(cmd: &CmdOpts) {
    let (d, fs) = fileio::get_data(cmd.input.as_path()).unwrap();

    if cmd.opts.debug {
        println!("Read success.");
        println!(
            "Signal duration: {} ms.",
            1000. * d.len() as f32 / fs as f32
        );
        println!("Sample rate: {} Hz.", fs);
    }

    // Chunk length requirements.
    const PEAK_FINDING_OVERLAP: usize = 2;
    let take = (cmd.opts.segment_size * 1e-3 * fs as f32) as usize;
    let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;
    let total_len = d.len();

    let (mut send, mut recv) = Segmenter::split(take, peek);

    // Write to the segmenter. This is in a separate thread, so that 
    // data could be collected from a sensor, as the data is processed.
    let t = thread::spawn(move || {
        for val in d.into_iter() {
            send.push(val);
        }
    });

    // Receive from the channel, and process.
    let pb = indicatif::ProgressBar::new(total_len as u64).with_style(
        indicatif::ProgressStyle::default_bar()
            .template("Analysing audio: {bar:40.cyan/blue} {percent:>3}% [eta: {eta}] [elasped: {elapsed}] {msg}")
            .progress_chars("##-"),
    );

    let mut identifier = analysis::BubbleIdentifier::new(&cmd.opts, fs);
    let mut joiner = summary::Joiner::new(&cmd.opts);

    // Count up from one.
    for idx in 1.. {
        pb.inc(take as u64);
        if pb.position() > pb.length() {
            pb.finish();
            pb.set_message("✔");
            break;
        }

        // Process chunk
        if let Ok(chunk) = recv.pop_segment() {
            
            let mut s = identifier.cwt(chunk);
            identifier.threshold(&mut s);
            if cmd.scaleograms {
                fileio::export_scaleogram(&s, cmd.out_dir.as_path(), idx);
            }
            let b = identifier.find_bubbles(&s);
            joiner.append(idx as isize, &b);
        }
    }

    let data = joiner.get_joined();

    match cmd.out_type {
        OutputType::Svg => fileio::plot_bubble_data(&data, cmd.out_dir.as_path(), 0)
            .expect("Bubble data could not be plotted"),
        OutputType::Csv => fileio::export_bubble_data(&data, cmd.out_dir.as_path(), 0)
            .expect("Bubble data could not be written to a csv file"),
        OutputType::Both => {
            fileio::plot_bubble_data(&data, cmd.out_dir.as_path(), 0)
                .expect("Bubble data could not be plotted");
            fileio::export_bubble_data(&data, cmd.out_dir.as_path(), 0)
                .expect("Bubble data could not be written to a csv file");
        }
    }

    t.join().unwrap();
}
