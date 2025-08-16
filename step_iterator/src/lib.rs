use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    finished: bool,
}

impl<T> StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            finished: false,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current_value = self.current;

        // Check if current value is beyond the end
        if current_value > self.end {
            self.finished = true;
            return None;
        }

        // Calculate next value
        let next_value = self.current + self.step;
        
        // If the next value would exceed the end, this is our last iteration
        if next_value > self.end {
            self.finished = true;
        } else {
            self.current = next_value;
        }

        Some(current_value)
    }
}