use std::fs::File;
pub fn open_file(s: &str) -> File {
     File::open(s).unwrap()
    // let res= match file{
    //     Ok(fl)=>fl,
    //     Err(_)=>panic!("file does not exist"),
    // };
    // res
}