1. La estructura Command por dentro es bastante compleja y grande, principalmente porque tiene que lidiar con las diferencias entre sistemas operativos (como Windows y Linux/Unix) de una manera muy eficiente y segura.

2. La variable command solo debe recibir el comando, los argumentos van en separado. Si actualmente ingreso "echo hola" dara error, con "ls" no pasa eso

3. El salto de linea funciona como una llamada al buffer de salida para asi imprimir en la pantalla, pero si usamos print! donde este omite los saltos de linea entonces el buffer no sabe cuando imprimir, para eso se usa el llamado stdout().flush() y se le dice a Rust "No me importa que no haya un salto de linea, vacia la sala de espera ahora mismo y pintalo en la pantalla"

4. Al usar el shell, spawn() le dice al SO "lanza este proceso hijo" y tambien hay que tener en cuenta que spawn es asincrono, apenas lanza el proceso hijo sigue el loop sin esperar que el hijo termine. El wait() le dice "para tu propia ejecucion aqui mismo. quedate congelado hasta que el proceso hijo termine de hacer su trabajo y muera". Tambien asi evitamos los procesos Zombies donde el hijo muere pero sigue ocupando espacio en la tabla de procesos y el padre no hizo el wait(). Osea basicamente el wait() tambien le sirve al padre para ver los resultados de su hijo

5. 
