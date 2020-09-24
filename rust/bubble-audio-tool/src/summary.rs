use crate::config;
use crate::fileio;
use std::path::PathBuf;

pub struct Joiner {
    segment_size: f32,
    data: Vec<(f32, f32)>,
    out_dir: PathBuf,
}

impl Joiner {
    pub fn new(opt: &config::Opt) -> Joiner {
        Joiner {
            segment_size: opt.segment_size,
            data: Vec::new(),
            out_dir: opt.out_dir.to_owned()
        }
    }
    pub fn append(&mut self, idx: isize, data: &[(f32, f32)]) {
        let size = self.segment_size;
        let iter = data.iter().map(|&(r,t)| {
            (r,t+size*(idx as f32))
        });
        self.data.extend(iter)
    }
    pub fn summarise(&self) {
        fileio::export_bubble_data(&self.data, self.out_dir.as_path(), 0);
    }
}
