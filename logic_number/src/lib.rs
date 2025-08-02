pub fn number_logic(num: u32) -> bool {
    let nums = num.to_string();
    let mut res=0;
    for i in nums.chars(){
       let b=i.to_string().parse::<u32>().unwrap();
       let l = nums.len();
        res+=b.pow(l.try_into().unwrap());
    }



// println!("{res},{num}");
    res==num

}