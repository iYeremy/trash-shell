# trash — A Tiny Shell written in Rust 🗑️ :3
trash es un prototipo de interprete de comandos (shell) minimalista desarrollado en Rust como un proyecto para aprender y reirme un poco. Su nombre es una parodia de bash. El objetivo principal fue divertirse mientras se exploraba la gestión de procesos en Linux/Unix y el estricto sistema de tipos y memoria de Rust.

## Características Actuales
- Bucle Interactivo (REPL): Muestra el prompt personalizado trash :3 >> de manera sutil e interactiva.

- Comandos Internos (Built-ins): Gestion propia de comandos que alteran el entorno del shell (como cd y exit) sin delegar en procesos externos.

- Ejecución Asincrona Controlada: Clonacion y lanzamiento de procesos hijos a través del sistema operativo para herramientas externas (ls, cat, grep, etc.).

- Inmortalidad ante Errores: Control de panics frente a comandos inexistentes o entradas vacias (presionar Enter por accidente xd).

- Manejo Eficiente de Memoria: Tokenizacion del texto mediante iteradores ligeros (SplitWhitespace), evitando copias innecesarias en la RAM.

## Diagrama de Flujo
El corazón de trash comparte la misma arquitectura lógica que shells profesionales como Bash o Zsh:

             [ Usuario escribe un comando ]
                           │
             ▲             ▼
             │     ¿Es un Built-in? (cd, exit...)
             │      /            \
             │    SÍ              NO
             │    /                \
    [ Lo ejecuta el ]        [ Hace .spawn() ]
    [ propio Padre  ]        [ Crea Proceso Hijo ]
             │                        │
             │                        ▼
             │               [ Padre hace .wait() ]
             │                        │
             └─────────────◄──────────┘


## Notas de Desarrollo
1. El ciclo de vida y los Procesos Zombie 
Cuando usas .spawn(), el sistema operativo lanza un proceso hijo de forma asíncrona en segundo plano. Al terminar, el hijo entra en estado "Zombie": libera su RAM pero mantiene su ID (PID) en la tabla de procesos esperando que el padre reclame su código de salida. Usar .wait() congela temporalmente al padre hasta que el hijo muere, recolectando sus datos finales y limpiando la tabla para evitar agotar los PIDs del sistema.

2. El misterio de cd e Inviolabilidad del Entorno 
Un proceso hijo NUNCA puede modificar el entorno o la ubicación de su proceso padre. Si cd fuera un programa externo en /bin/cd, se ejecutaría en un proceso hijo, cambiaría la ruta dentro de su propia memoria aislada, y al morir, dejaría al padre exactamente en el mismo directorio. Por eso, comandos como cd tienen que ser interceptados con un match y ejecutados de forma interna por el propio shell mediante la syscall std::env::set_current_dir.

3. Forzando la salida con .flush() 
Por defecto, la terminal almacena el texto en un búfer de salida intermedio y solo lo pinta en la pantalla cuando detecta un salto de línea (\n). Como el prompt utiliza print! (que omite el salto de línea para dejar al usuario escribir en la misma fila), el búfer se queda esperando. stdout().flush().expect(...) vacía manualmente esa sala de espera obligando al sistema a pintar el texto inmediatamente.

4. Mutabilidad Local en Funciones
En Rust, la mutabilidad pertenece a la variable (contenedor) y no a los datos en sí. Al pasar el iterador parts a la función execute_cd(mut args: ...), se transfiere la propiedad de la estructura. El main no necesita que la variable sea mutable, pero la función receptora declara localmente que sí modificará la estructura interna de la fila al consumir elementos con .next().

5. El Agujero Negro del Guion Bajo (_) 
El compilador de Rust marca ciertas funciones (como flush()) con el atributo #[must_use], obligándote a procesar su Result por seguridad. Cuando no nos interesa validar si la consola falló, asignar la ejecución a un guion bajo flotante (let _ = ...) actúa como una variable fantasma autoejecutable que descarta el valor de inmediato y silencia el warning de Cargo de forma limpia.

## Como ejecutar
Asegurate de tener instalado el toolchain de Rust (Cargo) en tu distro de Linux.

Clona este proyecto o copia el archivo main.rs.

Compila y arranca el shell ejecutando en tu terminal:
```bash
cargo run
```
Prueba comandos del sistema (ls -la, uname -a), navega con cd, presiona Enter vacío de forma segura, y cuando quieras terminar, escribe exit.

## Cosas a futuro (no)
Tuberías (Pipes |): Conectar la salida estándar (stdout) de un proceso hijo directamente a la entrada (stdin) del siguiente.

Prompt Dinámico: Modificar trash :3 >> para que muestre de forma elegante la ruta actual (std::env::current_dir) en la que está parado el shell.

Redirecciones (> y <): Permitir volcar los resultados de comandos directamente a archivos de texto físicos.
