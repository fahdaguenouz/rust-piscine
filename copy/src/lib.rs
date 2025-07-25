pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let abs_c = c.abs() as f64;
    (c,(c as f64).exp(),abs_c.ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res = String::new();
    for ch in a.split_whitespace() {
        if let Ok(n) = ch.parse::<f64>() {
            let e = n.exp();
            res += &format!("{} ", e);
        }
    }
    res.pop(); 
    (a, res)
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res=Vec::new();
    for nbr in b.iter(){
        res.push((*nbr as f64).ln());
    }
    
    (b,res)
}