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
        let up = i.to_ascii_uppercase();
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
