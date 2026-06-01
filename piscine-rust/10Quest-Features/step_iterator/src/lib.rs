use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T: Add<Output = T> + PartialOrd + Copy> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { current: beg, end, step }
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current <= self.end {
            let val = self.current;
            self.current = self.current + self.step;
            Some(val)
        } else {
            None
        }
    }
}
