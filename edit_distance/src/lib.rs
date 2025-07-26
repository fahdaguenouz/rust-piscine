pub fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let len_a = a_chars.len();
    let len_b = b_chars.len();

    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

 
    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; 
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j],       
                    std::cmp::min(
                        dp[i][j - 1],   
                        dp[i - 1][j - 1], 
                    ), 
                );
            }
        }
    }

    dp[len_a][len_b]
}