use std::io;
use std::process::Command;

fn main() {
    println!("Wpisz 1 aby dostać niespodziankę:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "1" {
        Command::new("cmd")
            .args(["/C", "start", "ptak.jpg"])
            .spawn()
            .unwrap();
    }
}