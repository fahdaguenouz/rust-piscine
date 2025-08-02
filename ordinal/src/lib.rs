pub fn num_to_ordinal(x: u32) -> String {
    let mut res = String::new();
    let dig=x%10;
    if 11<=x%100&&x%100<=13{
        res =format!("{x}th");
    }else{
        res=match dig{
            1=>format!("{x}st"),
            2=>format!("{x}nd"),
            3=>format!("{x}rd"),
            _=>format!("{x}th"),
        }
    }
    res

}