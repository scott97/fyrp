#![feature(test)] // for benchmarks.
#![feature(box_syntax)] // for box.

extern crate test; // for benchmarks.

#[macro_use]
extern crate approx;

mod analysis;
mod config;
mod conv;
mod cwt;
mod fileio;
mod iter;
mod mean_shift_clustering;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use structopt::StructOpt;

use config::*;
use cwt::alg;
use cwt::alg::Cwt;
use cwt::wavelets;



use winapi_util::console::{Console, Color, Intense};


fn main() {
    let opt = Opt::from_args();

    if opt.debug {
        let mut con = Console::stdout().unwrap();
        con.fg(Intense::Yes, Color::Magenta).unwrap();
        println!("Debug mode enabled.");
        con.reset().unwrap();
        println!("Configuration: {:#?}",&opt);
    }

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

    let frequency_bands: Vec<_> = iter::rangef(
        opt.min_radius * 1e-3,
        opt.max_radius * 1e-3,
        opt.radius_resolution * 1e-3,
    )
    .map(analysis::to_freq)
    .collect();

    if opt.debug {
        println!("Lowest frequency: {}", frequency_bands.first().unwrap());
        println!("Highest frequency: {}", frequency_bands.last().unwrap());
        println!("Number of frequency bands: {}", frequency_bands.len());
    }

    let mut cwt: Box<dyn Cwt<std::vec::IntoIter<f32>>> = match opt.cwt {
        CwtAlg::FftCpxFilterBank => box alg::FftCpxFilterBank::new(
            len,
            peek,
            |t| wavelets::soulti_cpx(t, 0.02),
            &frequency_bands,
            fs,
        ),
        CwtAlg::FftCpx => box alg::FftCpx::new(
            |t| wavelets::soulti_cpx(t, 0.02),
            [0., 50.],
            &frequency_bands,
            fs,
        ),
        CwtAlg::Fft => box alg::Fft::new(
            |t| wavelets::soulti(t, 0.02),
            [0., 50.],
            &frequency_bands,
            fs,
        ),
        CwtAlg::Standard => box alg::Standard::new(
            |t| wavelets::soulti(t, 0.02),
            [0., 50.],
            &frequency_bands,
            fs,
        ),
        CwtAlg::Simd => box alg::Simd::new(
            |t| wavelets::soulti(t, 0.02),
            [0., 50.],
            &frequency_bands,
            fs,
        ),
    };

    let pb = indicatif::ProgressBar::new(total_len as u64).with_style(
        indicatif::ProgressStyle::default_bar()
            .template("Analysing audio: {bar:40.cyan/blue} {percent:>3}% [eta: {eta}] [elasped: {elapsed}] {msg}")
            .progress_chars("##-"),
    );

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
        let mut s = if opt.parallel {
            cwt.process_par(&mut chunk.into_iter())
        } else {
            cwt.process(&mut chunk.into_iter())
        };
        if opt.scaleograms {
            fileio::export_scaleogram(&s,opt.out_dir.as_path(), idx);
        }
        analysis::threshold(&mut s, opt.threshold);
        let b = analysis::find_bubbles(&s, &frequency_bands, fs);
        fileio::export_bubble_data(&b,opt.out_dir.as_path(), idx);
    }

    t.join().unwrap();
}
