use queues::*;

pub fn rangef(start: f32, end: f32, step: f32) -> Rangef {
    Rangef {
        value: None,
        start,
        end,
        step,
    }
}

pub struct Rangef {
    value: Option<f32>,
    start: f32,
    end: f32,
    step: f32,
}

impl Iterator for Rangef {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Some(v) => {
                if v < self.end {
                    self.value = Some(v + self.step);
                    self.value
                } else {
                    None
                }
            }
            None => {
                self.value = Some(self.start);
                self.value
            }
        }
    }
}

impl DoubleEndedIterator for Rangef {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.value {
            Some(v) => {
                if v > self.start {
                    self.value = Some(v - self.step);
                    self.value
                } else {
                    None
                }
            }
            None => {
                self.value = Some(self.end);
                self.value
            }
        }
    }
}

pub struct CircIter<'a> {
    pub cbuf: &'a mut CircularBuffer<f32>,
    pub take: usize,
    pub peek: usize,
}

impl<'a> Iterator for CircIter<'a> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.take > 0 {
            self.take -= 1;
            Some(self.cbuf.remove().unwrap())
        } else if self.peek > 0 {
            self.peek -= 1;
            Some(self.cbuf.peek().unwrap())
        } else {
            None
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circ_iter() {
        let mut cbuf = CircularBuffer::<f32>::new(5);
        cbuf.add(1.);
        cbuf.add(2.);
        cbuf.add(3.);
        cbuf.add(4.);
        cbuf.add(5.);

        {
            let mut iter = CircIter {
                cbuf: &mut cbuf,
                take: 3,
                peek: 1,
            };

            assert_eq!(Some(1.), iter.next());
            assert_eq!(Some(2.), iter.next());
            assert_eq!(Some(3.), iter.next());
            assert_eq!(Some(4.), iter.next());
            assert_eq!(None, iter.next());
        }
        {
            let mut iter = CircIter {
                cbuf: &mut cbuf,
                take: 2,
                peek: 0,
            };

            assert_eq!(Some(4.), iter.next());
            assert_eq!(Some(5.), iter.next());
            assert_eq!(None, iter.next());
        }
    }

    #[test]
    fn test_forward() {
        let mut iter = rangef(0.0, 1.5, 0.5);
        assert_eq!(Some(0.0), iter.next());
        assert_eq!(Some(0.5), iter.next());
        assert_eq!(Some(1.0), iter.next());
        assert_eq!(Some(1.5), iter.next());
    }

    #[test]
    fn test_backward() {
        let mut iter = rangef(0.0, 1.5, 0.5);
        assert_eq!(Some(1.5), iter.next_back());
        assert_eq!(Some(1.0), iter.next_back());
        assert_eq!(Some(0.5), iter.next_back());
        assert_eq!(Some(0.0), iter.next_back());
    }
}
