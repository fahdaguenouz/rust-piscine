use pig_latin::*;

fn main() {
    println!("{}", pig_latin("square"));
    // println!("{}", pig_latin("apple"));
    // println!("{}", pig_latin("hello"));
    // println!("{}", pig_latin("square"));
    // println!("{}", pig_latin("xenon"));
    // println!("{}", pig_latin("chair"));
    // println!("{}", pig_latin("queen"));
}

#[test]
fn test_word_beginning_with_vowel() {
    assert_eq!(pig_latin("apple"), "appleay");
    assert_eq!(pig_latin("ear"), "earay");
    assert_eq!(pig_latin("igloo"), "iglooay");
    assert_eq!(pig_latin("object"), "objectay");
    assert_eq!(pig_latin("under"), "underay");
    assert_eq!(pig_latin("equal"), "equalay");
}

#[test]
fn test_word_beginning_with_consonant() {
    assert_eq!(pig_latin("queen"), "ueenqay");
    assert_eq!(pig_latin("square"), "aresquay");
    assert_eq!(pig_latin("pig"), "igpay");
    assert_eq!(pig_latin("koala"), "oalakay");
    assert_eq!(pig_latin("yellow"), "ellowyay");
    assert_eq!(pig_latin("xenon"), "enonxay");
    assert_eq!(pig_latin("qat"), "atqay");
    assert_eq!(pig_latin("chair"), "airchay");
    assert_eq!(pig_latin("therapy"), "erapythay");
    assert_eq!(pig_latin("thrush"), "ushthray");
    assert_eq!(pig_latin("school"), "oolschay");
    assert_eq!(pig_latin("british"), "itishbray");
}

#[test]
fn test_multiple_words() {
    assert_eq!(pig_latin("apple"), "appleay");
    assert_eq!(pig_latin("ear"), "earay");
    assert_eq!(pig_latin("igloo"), "iglooay");
    assert_eq!(pig_latin("qat"), "atqay");
    assert_eq!(pig_latin("school"), "oolschay");
    assert_eq!(pig_latin("therapy"), "erapythay");
}