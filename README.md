# PoW - Implementación Conceptual

La _Blockchain_ consiste de varios bloques referenciados. Dichos bloques tienen que estar validados de alguna forma. Ahí es donde entra en juego la técnica del _Proof of Work_ (PoW).

Se trata de un sistema de validación donde se hacen intentos aleatorios hasta que una condición se cumple. Se dice que es una prueba de trabajo porque se requiere de bastante poder computacional para hacer la mayor cantidad de intentos en el menor tiempo posible.

---

---

# La teoría...

## Función hash

En criptografía existe un tipo de funciones que se denominan _[funciones hash](https://es.wikipedia.org/wiki/Funci%C3%B3n_hash)_.

> La función hash tiene como entrada un conjunto de elementos, que suelen ser cadenas, y los convierte en un rango de salida finito, normalmente cadenas de longitud fija.

En un nivel conceptual, estas funciones hacen lo siguiente:

1. Reciben una entrada (generalmente un texto).
2. Procesan dicha entrada usando matemáticas y cosas así. A esta parte se la conoce también como "_digest_".
3. Devuelven como resultado una cadena de caracteres (otro texto). Esta tiene siempre la misma cantidad de caracteres. Este resultado se lo conoce como "_hash_".

La función hash que uso en este repo es [SHA512](https://es.wikipedia.org/wiki/SHA-2), la cual pertenece a un conjunto de funciones desarrolladas por la NSA.

Esta función va a devolver 128 caracteres hexadecimales, es decir, números del 0 al 9 y letras de la A a la F.

**¿Porque 128 caracteres?**

El 512 hace referencia a la cantidad de bits que tiene la salida. Todos estos bits los vamos a ubicar en grupos de 4, para poder convertirlos a caracteres hexadecimales (0-9 | A-F).

**¿Por qué agrupar de a 4 bits?**

La cantidad de símbolos del sistema hexadecimal es 16. 10 números (0 al 9) y 6 letras (A a la F).

El mayor número que podemos formar con 4 bits es 15. Si contamos el 0 podemos decir que tenemos 16 posibles combinaciones. Cada una de estas combinaciones va a corresponer a un símbolo.

```
0000 -> 0
0100 -> 4
1000 -> 8
1010 -> A
1111 -> F
```

## ¿Cuál sería el "trabajo" que hay que hacer?

Primero hay que tener todos los datos del bloque que queremos validar, el cual podemos pensarlo como un texto muy largo. Para el caso de este repo vamos a usar un texto arbitrario ingresado por el usuario y lo vamos a llamar `message`.

A este `message` lo vamos a introducir en la función SHA512, la cual nos va a devolver una cadena de caracteres.

Ahora bien, necesitamos que el hash `message` cumpla con una condición específica. Esta condición es: **_El hash tiene que empezar con 0_**.

Una forma de hacer esto es agregar al final de `message` un número cualquiera y volver a procesarlo con la función. En caso de que el nuevo hash no empiece con 0, agregamos un número distinto a `message` y volvemos a probar.

Esto se lo conoce como _Guess and check_ (Probar y comprobar). El nombre es autodescriptivo, porque estamos probando nuevos números al final de `message` y comprobando que el hash obtenido empiece con 0.

Una vez encontrado un número que agregado al final de `message` de como resultado un hash que comience con 0, el trabajo terminó.

En un sistema descentralizado, una vez encontrado el número correcto se notifica a los demás validadores en la red y ellos se encargan de comprobar que el número mágico es realmente el correcto.

El número mágico se lo conoce como `nonce`.

Una característica de esta forma de comprobación es que resulta muy difícil encontrar el `nonce` ya que requiere de intentar muchisimas veces número distintos para encontrar el correcto. Sin embargo, una vez encontrado, es muy fácil para los demás comprobar que un número es el índicado.

**Ejemplo**

1. Definimos como `message` el texto `hola`.
2. Usamos este texto como entrada de SHA512.
    > Resultado: e83e8535d6f689493e5819bd60aa3e5fdcba940e6d111ab6fb5c34f24f86496bf3726e2bf4ec59d6d2f5a2aeb1e4f103283e7d64e4f49c03b4c4725cb361e773
3. El resultado empieza por `e`, y necesitamos que empiece con `0`, entonces agregamos un número cualquiera al final del mensaje, separado por `:`.
4. Definimos el nuevo `message` como `hola:1`.
5. Usamos el nuevo texto como entrada de SHA512.
    > Resultado: 974c2f71df69ca6870f4a4cde8b4bc4a05342e580da08740b3ce83735f1e37984cd358307f313684fd95e8afe90c3835fe1265d83e4a8fe00a9a4baa2c735d71
6. El resultado sigue sin empezar por `0`, entonces sumamos uno al número final.
7. Definimos el nuevo `message` como `hola:2`.
8. Y así sucesivamente hasta que el mensaje cumpla con la condición.

El `nonce` para el mensaje `hola` es 262.

> Podés comprobarlo usando [esta app](https://emn178.github.io/online-tools/sha512.html). Si ingresas `hola:262`, se debería generar un hash que empieza con 0.

## ¿Por qué se hacen intentos aleatorios?

El sistema PoW está diseñado con el propósito de ser lo más difícil e ineficiente posible. En los sistemas descentralizados conviene que sea de esta manera por varias razones que van más allá de lo que hay en este repo.

En esta implementación usé un incrementador como posibles `nonce`. No obstante, se podría haber usado alguna librería que genere números aleatorios y probar con esos. A fines prácticos, es lo mismo, ya que no importa realmente el número, sino que al usarlo se cumpla la condición.

## Una condición un poco más difícil

La condición de que un hash empiece con 0 es bastante fácil de cumplir y no requiere de demasiado poder computacional. Por eso, la condición "real" es que el hash tiene que empezar con varios ceros.

Mientras más ceros exijamos al comienzo del hash, más complejo va a ser encontrar el `nonce`.

**¿Por qué?**

Podemos decir que dado un mensaje cualquiera, encontrar un `nonce` para que el hash empiece con solamente un 0 es fácil. Porque hay muchos números que pueden hacer que se cumpla la condición.
Por ejemplo, el mensaje `hola` tiene como `nonce`: 262, 473, 689, 1151, etc.

Ahora, si la condición es que el hash empiece con dos 0, se vuelve un poco más costoso, pero no imposible.
Los posibles `nonce` de `hola` serían: 39205, 78437, 170586, etc.

Ya con 3 ceros iniciales se vuelve mucho más costoso.
`nonce` para `hola` con 3 ceros iniciales: 1887566, 8762303, 9402711, etc.

Mientras más ceros inicales exijamos, más poder computacional necesitamos, porque hay cada vez "menos números" que hagan al hash cumplir la condición.
Se puede ver también que los números son cada vez más grandes.

Esto cumple con el propósito de validación de la Blockchain de Bitcoin. Hacer que encontrar el número sea lo más costoso posible.

## La Blockchain de Bitcoin

Hay muchas Blockchains, pero la más conocida es la de Bitcoin, la cual implementa el PoW.
Otras Blockchain como Ethereum y Cardano usan sistemas como [Proof of Stake](https://es.wikipedia.org/wiki/Prueba_de_apuesta).

En la realidad, para validar bloques en la Blockchain de Bitcoin se usa el algoritmo SHA256, el cual es muy parecido al que uso en este repo.

La cantidad de ceros requeridos para que un hash sea válido dependen de la dificultad de minería del momento. Esta dificultad se ajusta automáticamente para que, en promedio, siempre se tarde 10 minutos aproximadamente en encontrar el `nonce`.

---

# La práctica...

Esta prueba conceptual está hecha en [Rust](https://www.rust-lang.org/es).

Traté de que el código sea lo más simple y entendible posible.

Para usar esta implementación, hay que pasar dos argumentos. Uno que represente el mensaje y el otro la cantidad de ceros inicial.

```sh
cargo run <message> <zeros>
```

-   Para ingresar mensajes con espacio, es necesario rodearlo de comillas dobles ("").
-   La cantidad de ceros tiene que estar entre 0 y 32.
-   Cuando se encuentre el `nonce` se va a imprimir el hash y cuanto tiempo se estuvo buscando, junto con más info.

El código contiene comentarios y es bastante corto, así que no hace falta que lo explique acá. Andá y leelo directamente.

---

---

# Referencias

-   _[Bubble or Revolution: The Future of Bitcoin, Blockchains, and Cryptocurrencies](https://www.amazon.com/Blockchain-Bubble-Revolution-Present-Cryptocurrencies-ebook/dp/B07T13GP1Q)_ (libro)
