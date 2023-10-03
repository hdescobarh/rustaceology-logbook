// author: Hans D. Escobar H. (hdescobarh)
#![crate_name = "triples_pitagoricos"]
#![crate_type = "cdylib"]

/*
Reto #39: Triples pitagóricos
 Crea una función que encuentre todos los triples pitagóricos
 (ternas) menores o iguales a un número dado.
 - Debes buscar información sobre qué es un triple pitagórico.
 - La función únicamente recibe el número máximo que puede
   aparecer en el triple.
 - Ejemplo: Los triples menores o iguales a 10 están
   formados por (3, 4, 5) y (6, 8, 10).
 */

/*
 Un triple pitagórico es un conjunto de tres números enteros positivos que satisface el teorema de Pitágoras.
 Por trigonometría se tiene que:
 1. El máximo valor es la hipotenusa.
 2. El resultado de multiplicar un triple por una constante también es un triple.

Mi estrategia consiste en aplicar (1) para encontrar los dos valores restantes que satisfacen el triple;
posteriormente, a través de los divisores comunes (2) encontrar los triples menores asociados.
*/

#[cfg(test)]
mod tests {
    //use super::*;
}
