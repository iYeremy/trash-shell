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
        
        match command {
            // comandos built-in
            "cd" => execute_cd(args),
            "exit" => std::process:exit(0), // =return, cierra el proceso padre (shell) ahora mismo con el código de salida 0 (exito)

            // procesos hijos
            _ => {
                let mut command_child = Command::new(command).args(args).spawn().unwrap();
                command_child.wait(); // espera hasta que el proceso hijo termine de completarse para asi continuar el floop
            },
        }
    }
}

// de forma modular separo por funciones, asi el match queda mas limpio 

fn execute_cd(mut args: std::str::SplitWhitespace){ // recibe el iterador con los argumentos
                                                    // restantes, es mutable para que se pueda usar
                                                    // el .next

    // args.next() devuelve un Option, por lo tanto si solo se coloca cd sin argumentos, el next
    // devolvera un None
    if let Some(nuevo_destino) = args.next() { // tipo de match mas corto
        
        // se hace la syscall para mover el proceso actual a la ruta especifica en nuevo destino
        if let Err(e) = std::env::set_current_dir(nuevo_destino) {  // set_current_dir devuelve un
                                                                    // Result ya que posiblemente el
                                                                    // nombre del directorio de
                                                                    // errores
            println!("osh: cd: {}", e);
        }
    } else {
        // no he mirado como agregar variables de entorno como $HOME, coloque un simple print
        println!("osh: cd: se requiere un argumento");
    }
}
