use std::io;
use std::io::Write;

fn output_conversions(number: u32) -> () {
    println!("\nDEC: {}", number);
    println!("HEX: {:X}", number);
    println!("OCT: {:o}\n", number);
}

fn main() {
    loop {
        print!("Enter a number: ");
        io::stdout().flush().expect("Flush failed!");

        let mut scanned_number = String::new();
        io::stdin()
            .read_line(&mut scanned_number)
            .expect("Your input is not valid.");

        let number: u32 = scanned_number.trim().parse().expect("That is not a valid number!");
        output_conversions(number);
    }
}
