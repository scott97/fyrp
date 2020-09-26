#![feature(test)] // for benchmarks.
#![feature(box_syntax)] // for box.
#![feature(trait_alias)] // for trait _ = _.

extern crate test; // for benchmarks.

#[macro_use]
extern crate approx;

mod analysis;
mod config;
mod conv;
mod cwt;
mod fileio;
mod integration_tests;
mod iter;
mod mean_shift_clustering;
mod summary;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use structopt::StructOpt;
use winapi_util::console::{Color, Console, Intense};

fn main() {
    let opt = config::Opt::from_args();

    if opt.debug {
        let mut con = Console::stdout().unwrap();
        con.fg(Intense::Yes, Color::Magenta).unwrap();
        println!("Debug mode enabled.");
        con.reset().unwrap();
        println!("Configuration: {:#?}", &opt);
    }

    run(&opt);
}

fn run(opt: &config::Opt) {
    let (d, fs) = fileio::get_data(opt.input.as_path()).unwrap();

    if opt.debug {
        println!("Read success.");
        println!(
            "Signal duration: {} ms.",
            1000. * d.len() as f32 / fs as f32
        );
        println!("Sample rate: {} Hz.", fs);
    }

    // Chunk length requirements.
    const PEAK_FINDING_OVERLAP: usize = 2;
    let take = (opt.segment_size * 1e-3 * fs as f32) as usize;
    let peek = (50e-3 * fs as f32) as usize + PEAK_FINDING_OVERLAP;
    let len = take + peek;
    let total_len = d.len();

    // Channel
    let (tx, rx): (Sender<f32>, Receiver<f32>) = mpsc::channel();

    // Send chunks over channel. Prepare chunks with overlapping data.
    let t = thread::spawn(move || {
        // Iterator
        let mut iter = d.into_iter().peekable();

        // Read in data into the channel.
        'outer: loop {
            for _i in 0..take {
                match iter.next() {
                    Some(x) => tx.send(x).unwrap(),
                    None => break 'outer,
                }
            }
            for _i in 0..peek {
                match iter.peek() {
                    Some(x) => tx.send(*x).unwrap(),
                    None => break 'outer,
                }
            }
        }
    });

    // Receive from the channel, and process.
    let pb = indicatif::ProgressBar::new(total_len as u64).with_style(
        indicatif::ProgressStyle::default_bar()
            .template("Analysing audio: {bar:40.cyan/blue} {percent:>3}% [eta: {eta}] [elasped: {elapsed}] {msg}")
            .progress_chars("##-"),
    );

    let mut identifier = analysis::BubbleIdentifier::new(&opt, fs);
    let mut joiner = summary::Joiner::new(&opt);

    // Count up from one.
    for idx in 1.. {
        pb.inc(take as u64);
        if pb.position() > pb.length() {
            pb.finish();
            pb.set_message("✔");
            break;
        }

        // Receive a chunk of data
        let mut chunk = Vec::with_capacity(len);
        for _i in 0..len {
            chunk.push(rx.recv().unwrap())
        }

        // Process chunk
        let b = identifier.process(chunk);
        joiner.append(idx, &b);
    }

    let data = joiner.get_joined();
    fileio::export_bubble_data(&data, opt.out_dir.as_path(), 0)
        .expect("Bubble data could not be written to a csv file");
    fileio::plot_bubble_data(&data, opt.out_dir.as_path(), 0)
        .expect("Bubble data could not be plotted");

    t.join().unwrap();
}
