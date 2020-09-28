#![feature(test)] // for benchmarks.
#![feature(box_syntax)] // for box.
#![feature(trait_alias)] // for trait _ = _.

pub mod analysis;
pub mod summary;
pub mod config;
mod conv;
mod cwt;
mod iter;
mod mean_shift_clustering;
mod xcorr;


