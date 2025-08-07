pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
   let mut res: Vec<Box<u32>>=vec![]; 
   let nbrs = s.split_whitespace();
   println!("{:?}",nbrs);
   for i in nbrs {
        let value = if i.ends_with('k') {
            let num_part = &i[..i.len() - 1]; 
            match num_part.parse::<f32>() {
                Ok(n) => (n * 1000.0) as u32,
                Err(_) => continue,
            }
        } else {
            match i.parse::<u32>() {
                Ok(n) => n,
                Err(_) => continue,
            }
        };
        res.push(Box::new(value));
    }
   res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
   let mut res:Vec<u32> =vec![];
   for i in a.iter(){
    res.push(**i);
   }
   res
}