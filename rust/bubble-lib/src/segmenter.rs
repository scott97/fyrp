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

        (Sender { seg: seg.clone() }, Receiver { seg })
    }

    fn push(&mut self, val: f32) {
        self.queue
            .push(val)
            .expect("Could not push to the segmenter");
    }

    fn pop_segment(&mut self) -> Result<Vec<f32>, SegmentError> {
        let mut seg = self.queue.pop_n(self.segment_length)?;
        seg.extend(self.queue.peek_n(self.overlap)?.iter());
        Ok(seg)
    }
}

#[derive(Debug)]
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmenter_3_2() {
        let (mut send, mut recv) = Segmenter::split(3, 2);

        send.push(1.0);
        send.push(2.0);
        send.push(3.0);
        send.push(4.0);
        send.push(5.0);
        send.push(6.0);
        send.push(7.0);
        send.push(8.0);
        send.push(9.0);
        send.push(10.0);
        send.push(11.0);
        send.push(12.0);

        assert_eq!(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            recv.pop_segment().expect("unexpected segment error")
        );
        assert_eq!(
            vec![4.0, 5.0, 6.0, 7.0, 8.0],
            recv.pop_segment().expect("unexpected segment error")
        );
        assert_eq!(
            vec![7.0, 8.0, 9.0, 10.0, 11.0],
            recv.pop_segment().expect("unexpected segment error")
        );

        recv.pop_segment()
            .expect_err("expected segment error but did not receive one");
    }

    #[test]
    fn test_segmenter_4_4() {
        let (mut send, mut recv) = Segmenter::split(4, 4);

        send.push(1.0);
        send.push(2.0);
        send.push(3.0);
        send.push(4.0);
        send.push(5.0);
        send.push(6.0);
        send.push(7.0);
        send.push(8.0);
        send.push(9.0);
        send.push(10.0);
        send.push(11.0);
        send.push(12.0);

        assert_eq!(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            recv.pop_segment().expect("unexpected segment error")
        );
        assert_eq!(
            vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            recv.pop_segment().expect("unexpected segment error")
        );

        recv.pop_segment()
            .expect_err("expected segment error but did not receive one");
    }
}
