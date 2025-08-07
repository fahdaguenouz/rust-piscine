use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    value: RefCell<usize>, // Interior mutable
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: Rc::new(RefCell::new(Vec::new())),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<i32>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            *self.value.borrow_mut() = count;
            if percentage > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percentage
                ));
            }
        }
    }

    pub fn peek(&self, value: &Rc<i32>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }
}