pub fn fibonacci(n: usize)->u32 {
    let mut fibo = vec![0, 1];
    for i in 2..=n {
        let nt = fibo[i - 1] + fibo[i - 2];
        fibo.push(nt);
    }
fibo[n]
}