pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    let mut res="";
    if s.starts_with(prefix){
       res= s.strip_prefix(prefix)?;
    }else{
        return None;
    }


    Some(res)
}