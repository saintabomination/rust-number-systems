use std::io;

fn main() {
    let mut scanned_number = String::new();
    io::stdin()
        .read_line(&mut scanned_number)
        .expect("Your input is not valid.");

    let number: u32 = scanned_number.trim().parse().expect("That is not a valid number!");
    println!("{}", number);
}
