#[derive(Clone)]
pub struct RomanNumber {
    digits: Vec<char>, // stores Roman digits
    value: u32,        // store numeric value
}

// custom Debug to match expected output
use std::fmt;
impl fmt::Debug for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RomanNumber")
            .field(&self.digits)
            .finish()
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let digits = to_roman(n);
        RomanNumber { digits, value: n }
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;              // increment number
        let digits = to_roman(self.value);
        self.digits = digits.clone(); // update internal digits
        Some(RomanNumber { digits, value: self.value })
    }
}

fn to_roman(mut n: u32) -> Vec<char> {
    let mut result = Vec::new();
    let values = [
        (1000, 'M'), (900, 'C'), (500, 'D'), (400, 'C'),
        (100, 'C'), (90, 'X'), (50, 'L'), (40, 'X'),
        (10, 'X'), (9, 'I'), (5, 'V'), (4, 'I'), (1, 'I'),
    ];

    for &(val, ch) in values.iter() {
        while n >= val {
            n -= val;
            result.push(ch);
        }
    }

    result
}

