pub fn first_fifty_even_square() -> Vec<i32> {
let mut res:Vec<i32>=Vec::new();
        let  check= |nbr:i32,res:&mut Vec<i32>|{
            let sq=nbr.isqrt();
        
        if sq*sq==nbr{
            res.push(nbr);
        }
    };
    let mut i =2;
    while res.len()<50{
       if i%2==0{
         check(i ,&mut res);
       }
        i+=1;
    }
res
}
