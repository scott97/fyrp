use slice_queue::ReadableSliceQueue;
use slice_queue::SliceQueue;
use slice_queue::WriteableSliceQueue;
use std::cell::UnsafeCell;
use std::sync::Arc;

pub struct Segmenter {
    queue: SliceQueue<f32>,
    segment_length: usize,
    overlap: usize,
}

unsafe impl Sync for Sender {}
unsafe impl Send for Sender {}

pub struct Sender {
    seg: Arc<UnsafeCell<Segmenter>>,
}

pub struct Receiver {
    seg: Arc<UnsafeCell<Segmenter>>,
}

impl Sender {
    pub fn push(&mut self, val: f32) {
        unsafe { &mut *self.seg.get() }.push(val);
    }
}

impl Receiver {
    pub fn pop_segment(&mut self) -> Result<Vec<f32>, SegmentError> {
        unsafe { &mut *self.seg.get() }.pop_segment()
    }
}

impl Segmenter {
    pub fn split(segment_length: usize, overlap: usize) -> (Sender, Receiver) {
        let seg = Arc::new(UnsafeCell::new(Segmenter {
            queue: SliceQueue::new(),
            segment_length,
            overlap,
        }));

        (Sender { seg: seg.clone() }, Receiver { seg: seg })
    }

    fn push(&mut self, val: f32) {
        self.queue.push(val).expect("Could not push to the segmenter");
    }

    fn pop_segment(&mut self) -> Result<Vec<f32>, SegmentError> {
        let mut seg = self.queue.pop_n(self.segment_length)?;
        seg.extend(self.queue.peek_n(self.overlap)?.iter());
        Ok(seg)
    }
}

pub enum SegmentError {
    PopError(),
    PeekError(),
}

impl From<Vec<f32>> for SegmentError {
    fn from(_e: Vec<f32>) -> Self {
        SegmentError::PopError()
    }
}
impl From<&[f32]> for SegmentError {
    fn from(_e: &[f32]) -> Self {
        SegmentError::PeekError()
    }
}
