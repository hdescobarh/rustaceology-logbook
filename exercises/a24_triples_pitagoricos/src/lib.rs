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
 Un triple pitagórico es un conjunto de números enteros que satisface el teorema de Pitágoras.
 Por trigonometría sabemos que:
 - el máximo valor es la hipotenusa.
 - los múltiplos comunes de un triple también es un triple.

Mi propuesta consiste en encontrar los dos valores restantes que satisfacen el triple, y encontrar
los divisores comunes para con ellos calcular los triples menores.
  */

#[cfg(test)]
mod tests {
    //use super::*;
}
