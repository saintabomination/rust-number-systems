use std::io;

fn main() {
    let mut scannedNumber = String::new();
    io::stdin()
        .read_line(&mut scannedNumber)
        .expect("Your input is not valid.");
    
    println!("{}", scannedNumber);
}
