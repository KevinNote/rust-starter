use std::io::{stdin, stdout, Write};

fn main() {
    let mut read: String = String::new();
    print!("Enter a string: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut read);
    println!("{}", read);

    print!("Enter a i32: ");
    stdout().flush().unwrap();
    read.clear();
    stdin().read_line(&mut read);
    println!("Receive {}", read.trim());
    let i_read: i32 = read.trim().parse::<i32>().unwrap_or(0);
    println!("I32: {}", i_read);
}
