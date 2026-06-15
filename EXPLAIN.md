1. La estructura Command por dentro es bastante compleja y grande, principalmente porque tiene que lidiar con las diferencias entre sistemas operativos (como Windows y Linux/Unix) de una manera muy eficiente y segura.

2. La variable command solo debe recibir el comando, los argumentos van en separado. Si actualmente ingreso "echo hola" dara error, con "ls" no pasa eso

3. El salto de linea funciona como una llamada al buffer de salida para asi imprimir en la pantalla, pero si usamos print! donde este omite los saltos de linea entonces el buffer no sabe cuando imprimir, para eso se usa el llamado stdout().flush() y se le dice a Rust "No me importa que no haya un salto de linea, vacia la sala de espera ahora mismo y pintalo en la pantalla"

4. Al usar el shell, spawn() le dice al SO "lanza este proceso hijo" y tambien hay que tener en cuenta que spawn es asincrono, apenas lanza el proceso hijo sigue el loop sin esperar que el hijo termine. El wait() le dice "para tu propia ejecucion aqui mismo. quedate congelado hasta que el proceso hijo termine de hacer su trabajo y muera". Tambien asi evitamos los procesos Zombies donde el hijo muere pero sigue ocupando espacio en la tabla de procesos y el padre no hizo el wait(). Osea basicamente el wait() tambien le sirve al padre para ver los resultados de su hijo

5. Se busco una forma para que los argumentos se pudieran leer, se uso funciones de String para hacer una fila de cadenas de texto y despues irlas tratando, en este caso command es el primer campo y los argumentos son los posteriores, entonces al agregar el campo a command los argumentos no deben tener este mismo campo.

6. Hay comandos que el shell no puede enviar a otros procesos. Se trata de cosas que afectan internamente del Shell y por tanto deben ser incorpordas dentro del propio shell como casos externos.

7. Un proceso hijo NUNCA puede modificar el entorno de su proceso padre. Cuando se hace el Command::new(command).spawn() lo que hacemos en que el proceso padre (shell) invocara al proceso hijo (command) y este proceso hijo llamara al binario (ejem /bin/ls), este hijo hereda la carpeta actual del padre hace su trabajo y despues muere. cd es un comando interno, es decir debe ser tratado desde el propio proceso padre osea del shell ya que no existe un binario independiente y es normal porque el hijo SI cambia su propio directorio, pero cuando muere el papa sigue en el base de donde estaba, ya que no se puede incumplir la regla 6. Entonces es necesario tratar estos comandos internos con un match

8. Las funciones son como un mundo aparte, un parametro que recibe una variable inmutable pero en este propio parametro tratamos a la variable como mutable funciona

9. _ es un comodin donde permite todo en un match o una variable que se autodestruye inmediatamente con si fuera fantasma, Result tiene un campo #[must_use] que quiere decir que debe ser usado, viene perfectecto para cuando no nos importa usar algo ni revisarlo
Flujo del shell

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


