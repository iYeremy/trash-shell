1. la estructura Command por dentro es bastante compleja y grande, principalmente porque tiene que lidiar con las diferencias entre sistemas operativos (como Windows y Linux/Unix) de una manera muy eficiente y segura.

2. la variable command solo debe recibir el comando, los argumentos van en separado. Si actualmente ingreso "echo hola" dara error, con "ls" no pasa eso
