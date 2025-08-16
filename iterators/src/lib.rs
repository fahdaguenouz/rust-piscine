#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
    started: bool, // track whether we've yielded the first value yet
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n, started: false }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }

        // First call: just return the starting value
        if !self.started {
            self.started = true;
            return Some(self.v);
        }

        // Already started, compute next step
        if self.v == 1 {
            return None;
        }

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(self.v)
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    // Subtract 1 because we don’t count the starting value
    Collatz::new(n).count() - 1
}
