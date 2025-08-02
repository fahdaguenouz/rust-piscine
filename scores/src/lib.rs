pub fn score(str: &str) -> u64 {
    let mut count: u64 = 0;
    let score1 = "AEIOULNRST";
    let score2 = "DG";
    let score3 = "BCMP";
    let score4 = "FHVWY";
    let score5 = "K";
    let score8 = "JX";
    let score10 = "QZ";
    for i in str.chars() {
        // let up = i.to_ascii_uppercase();
        //   count += match ch {
        //     'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        //     'D' | 'G' => 2,
        //     'B' | 'C' | 'M' | 'P' => 3,
        //     'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        //     'K' => 5,
        //     'J' | 'X' => 8,
        //     'Q' | 'Z' => 10,
        //     _ => 0, 
        // };
        if score1.contains(up) {
            count += 1;
        } else if score2.contains(up) {
            count += 2;
        } else if score3.contains(up) {
            count += 3;
        } else if score4.contains(up) {
            count += 4;
        } else if score5.contains(up) {
            count += 5;
        } else if score8.contains(up) {
            count += 8;
        } else if score10.contains(up) {
            count += 10;
        }
    }
    count
}
