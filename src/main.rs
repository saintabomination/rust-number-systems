use std::io;
use std::io::Write;

fn output_conversions(number: i32) -> () {
    println!("\nDEC: {}", number);
    println!("HEX: {:X}", number);
    println!("OCT: {:o}\n", number);
}

fn main() {
    println!("Number Conversion Program");
    println!("Use 'q' to quit.\n");

    loop {
        print!("Enter a number: ");
        io::stdout().flush().expect("Flush failed!");

        let mut scanned_number = String::new();
        io::stdin()
            .read_line(&mut scanned_number)
            .expect("Your input is not valid.");

        if scanned_number.trim() == "q" {
            break;
        }

        let number: i32 = scanned_number.trim().parse().expect("That is not a valid number!");
        output_conversions(number);
    }

    println!("Thank you for using the program.");
}
