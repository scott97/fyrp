#![feature(test)] // for benchmarks.

extern crate test; // for benchmarks.

#[macro_use]
extern crate approx;

mod analysis;
mod conv;
mod cwt;
mod iter;

use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use cwt::alg;
use cwt::alg::Cwt;
use cwt::wavelets;

fn get_data() -> Option<(Vec<f32>, u32)> {
    let input_file = Path::new("data.wav");
    let mut inp_file = File::open(input_file).unwrap();
    let (header, data) = wav::read(&mut inp_file).unwrap();
    let fs = header.sampling_rate;

    // Remap to range -1.0 to 1.0
    if let wav::BitDepth::Sixteen(raw_signal) = data {
        println!("Read success (i16)");
        let y = raw_signal
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect();
        Some((y, fs))
    } else {
        None
    }
}

// Write scaleogram data to a csv file
fn export_scaleogram(s: &Vec<Vec<f32>>, idx: usize) {
    let name = format!("scaleogram{}.csv", idx);

    println!(
        "Exporting scaleogram ({}×{}) as file: {}",
        s.len(),
        s[0].len(),
        name
    );

    let path = Path::new(&name);
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for row in s.into_iter() {
        let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
        wtr.write_record(&text_vec).unwrap();
    }
    wtr.flush().unwrap();
}

fn main() {
    if let Some((d, fs)) = get_data() {
        println!("Signal length {}", d.len());
        println!("Sample rate {}", fs);

        // Chunk length requirements.
        let len = (0.100 * fs as f32) as usize;
        let peek = (50e-3 * fs as f32) as usize;
        let take = len - peek;

        // Channel
        let (tx, rx): (Sender<f32>, Receiver<f32>) = mpsc::channel();

        // Send chunks over channel. Prepare chunks with overlapping data.
        let tx_thread = tx.clone(); // Threads take a copy of the sender.
        let t = thread::spawn(move || {

            // Iterator
            let mut iter = d.into_iter().peekable();

            // Read in data into the channel.
            loop {
                for _i in 0..take {
                    match iter.next() {
                        Some(x) => tx_thread.send(x).unwrap(),
                        None => panic!("Ran out of data"),
                    }
                }
                for _i in 0..peek {
                    match iter.peek() {
                        Some(x) => tx_thread.send(*x).unwrap(),
                        None => panic!("Ran out of data"),
                    }
                }
            }
        });

        // Receive from the channel, and process.
        let frequency_bands: Vec<f32> = iter::rangef(1000.0, 9000.0, 20.0).collect();
        let mut cwt = alg::FftCpxFilterBank::new(
            len,
            take,
            |t| wavelets::soulti_cpx(t, 0.02),
            &frequency_bands,
            fs,
        );

        for idx in 1.. { // Count up from one.

            // Receive a chunk of data
            let mut chunk = Vec::with_capacity(len);
            for i in 0..len {
                chunk.push(rx.recv().unwrap())
            }

            // Process chunk
            let mut s = cwt.process_par(&mut chunk.into_iter());
            // analysis::threshold(&mut s, 100.);
            export_scaleogram(&s, idx);
        }

        t.join().unwrap();
    }
}
