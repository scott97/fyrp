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
