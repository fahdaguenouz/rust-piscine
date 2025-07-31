use std::path::Path;
use std::fs::OpenOptions;
 use std::io::Write;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
   let file=OpenOptions::new().read(true).append(true).write(true).create(true).open(path);
   match file {
     Ok(mut f) => {
            if let Err(e) = f.write_all(content.as_bytes()) {
                panic!("Failed : {}", e);
            }
        },
        Err(e) => panic!("Failed: {}", e),
   }
}