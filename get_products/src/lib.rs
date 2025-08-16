pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::new();

    for i in 0..arr.len() {
        let mut mul = 1;
        for j in 0..arr.len() {
            if i != j {
                mul *= arr[j];
            }
        }
        res.push(mul);
    }

    res
}