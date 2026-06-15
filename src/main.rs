use std::io::stdin; // importar input
use std::process::Command; // importar struct Command 
fn main() {
    let mut input = String::from("");
    stdin().read_line(&mut input).unwrap();

    let command = input.trim(); // .trim elimina los espacios en blanco, tabulaciones y saltos de linea

    Command::new(command).spawn().unwrap();
}   
