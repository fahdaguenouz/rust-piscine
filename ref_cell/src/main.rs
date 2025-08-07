use std::rc::Rc;

use ref_cell::*;

fn main() {
    let v = Rc::new(1); // we have one reference to this Rc

    // initialize the tracker, with the max number of
    // called references as 10
    let mut  track = Tracker::new(10);

    let _v = Rc::clone(&v); // |\
    let _v = Rc::clone(&v); // | -> increase the Rc to 4 references
    let _v = Rc::clone(&v); // |/

    // take a peek of how much we already used from our quota
    track.peek(&v);

    let _v = Rc::clone(&v); // |\
    let _v = Rc::clone(&v); // |  -> increase the Rc to 8 references
    let _v = Rc::clone(&v); // | /
    let _v = Rc::clone(&v); // |/

    // this will change the tracker's inner value
    // and make a verification of how much we already used of our quota
    track.set_value(&v);

    let _v = Rc::clone(&v); // increase the Rc to 9 references
    let _v = Rc::clone(&v); // increase the Rc to 10 references, the maximum we allow

    track.set_value(&v);

    let _v = Rc::clone(&v); // surpass the maximum allowed references

    track.peek(&v);
    track.set_value(&v);

    track
        .messages
        .borrow()
        .iter()
        .for_each(|msg| println!("{}", msg));
}





#[test]
fn test_one() {
    let expected_messages = [
        "Info: This value would use 40% of your quota",
        "Warning: You have used up over 80% of your quota!",
        "Warning: You have used up over 100% of your quota!",
        "Error: You can't go over your quota!",
    ];

    let value = Rc::new(42);

    let  track = Tracker::new(5);
    let _v = Rc::clone(&value);
    track.peek(&value); // 40%
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    track.set_value(&value); // 80%
    let _v = Rc::clone(&value);
    track.set_value(&value); // 100%
    let _v = Rc::clone(&value);
    track.set_value(&value); // >100%

    assert_eq!(track.messages.borrow().as_slice(), expected_messages);
}

#[test]
fn test_two() {
    let value = Rc::new(100);

    let track = Tracker::new(12);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);

    track.set_value(&value);

    let _v = Rc::clone(&value);

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 83% of your quota!"
    );

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 83% of your quota"
    );

    let _v = Rc::clone(&value);
    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 91% of your quota"
    );

    let _v = Rc::clone(&value);
    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 100% of your quota!"
    );

    let _v = Rc::clone(&value);

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 108% of your quota"
    );

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Error: You can't go over your quota!"
    );
}