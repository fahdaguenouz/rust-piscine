pub fn stars(n: u32) -> String {
let star="*";
star.repeat(2_i32.pow(n).try_into().unwrap())
}