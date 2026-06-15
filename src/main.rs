use std::io::stdin; // importar i/o
use std::io::stdout;
use std::io::Write;
use std::process::Command; // importar struct Command 
fn main() {
    loop{
        print!("osh :3 >>"); // imprime sin salto de linea
        stdout().flush(); 
        let mut input = String::from("");
        stdin().read_line(&mut input).unwrap();

        let command = input.trim(); // .trim elimina los espacios en blanco, tabulaciones y saltos de linea

        Command::new(command).spawn().unwrap();
    }
}
