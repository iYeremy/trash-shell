use std::io::stdin; // importar i/o
use std::io::stdout;
use std::io::Write; // necesario para el flush
use std::process::Command; // importar struct Command 
fn main() {
    loop{
        print!("osh :3 >>"); // imprime sin salto de linea
        stdout().flush(); // vacia el bufer de salida. forzando a que el texto se imprima inmediatamente en la consola  
        let mut input = String::from("");
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace(); // separador de espacios (iterador)
                                                         
        let command = parts.next().unwrap(); // toma el primer campo de parts y se lo asigna a
                                             // command, luego quita este campo de parts

        let args = parts; // variable inmutable para argumentos restantes
        
        let mut command_child = Command::new(command).args(args).spawn().unwrap();

        command_child.wait(); // espera hasta que el proceso hijo termine de completarse para asi continuar el floop
    }
}
