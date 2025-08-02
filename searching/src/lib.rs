pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut  index:Option<usize>=None;
    for (i , nbr) in array.iter().enumerate(){
        if *nbr==key{
            index=Some(i);
        }
    }

   index
}