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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let mut iter = rangef(0.0, 1.5, 0.5);
        assert_eq!(Some(0.0),iter.next());
        assert_eq!(Some(0.5),iter.next());
        assert_eq!(Some(1.0),iter.next());
        assert_eq!(Some(1.5),iter.next());
    }

    #[test]
    fn test_backward() {
        let mut iter = rangef(0.0, 1.5, 0.5);
        assert_eq!(Some(1.5),iter.next_back());
        assert_eq!(Some(1.0),iter.next_back());
        assert_eq!(Some(0.5),iter.next_back());
        assert_eq!(Some(0.0),iter.next_back());
    }
}


// Note:
// similar to crate itertools_num::linspace
// https://docs.rs/itertools-num/0.1.3/itertools_num/fn.linspace.html
// but I think mine is better for what I need