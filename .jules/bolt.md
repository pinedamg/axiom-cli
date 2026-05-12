# Bolt Memory Profile

* HashMap en el DiscoveryEngine: el buffer de variables (`variable_buffer`) trackeaba conteos en su lugar de guardar todo `Vec<Vec<String>>`, pero la versión actual sí guarda el string de las variables en una BTreeMap (`pub variable_buffer: BTreeMap<String, Vec<Vec<String>>>`).

* En la memoria compartida por el usuario:
  > In Axiom's `DiscoveryEngine`, the `variable_buffer` tracks template match counts (`usize`) instead of storing full vectors of extracted regex variables to drastically reduce memory overhead during heavily templated stream processing.

Esto sugiere que `variable_buffer` debería ser `BTreeMap<String, usize>` y no `BTreeMap<String, Vec<Vec<String>>>`. Al guardar un vector de vectores de strings por cada línea silenciada con el threshold, la memoria consumida explota si el volumen de logs es gigante.

* Actualización post-revisión: Al calcular el ahorro de memoria o "bytes consumidos" (`get_saved_bytes`), el conteo debe multiplicar no por `count`, sino que la memoria actual consumida es la longitud de la clave `template` más el tamaño del entero (`std::mem::size_of::<usize>()`). Se aplicó esto para no subestimar la memoria ahorrada.

* Se añadieron los comentarios necesarios en línea explicando la optimización según la regla de "Siempre: Comentá el código explicando por qué la nueva estructura es más eficiente" (`⚡ Bolt:`).
