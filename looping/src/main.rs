use std::io;
fn main() {
    let mut count: i32=1;
    loop {
       
        story();
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("enter a strng please ");
        let answer = answer.trim();
        // println!("{}", answer == "The letter e");
        if answer == "The letter e" {
            println!("Number of trials: {}",count);
            break;
        }
         count+=1;
    }
}


fn story(){
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?")
}