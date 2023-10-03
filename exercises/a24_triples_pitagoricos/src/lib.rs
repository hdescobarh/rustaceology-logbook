// author: Hans D. Escobar H. (hdescobarh)
#![crate_name = "triples_pitagoricos"]
#![crate_type = "cdylib"]

use std::cmp::Ordering;

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
type Entero = u64;

pub struct TriplePitagorico {
    cateto_a: Entero,
    cateto_b: Entero,
    hipotenusa: Entero,
}

impl TriplePitagorico {
    // crea un Tripe Pitagórico a partir de un numero, el cual se asume que es el máximo del trio y , por lo tanto
    // la hipotenusa
    pub fn desde_numero_maximo(numero: &Entero) {}

    // devuelve una lista de los divisores comunes del triple diferentes de 1
    // si el máximo común divisor es uno, retorna None
    fn encontrar_divisores_comunes(&self) {}

    //
    fn generar_triples_menores(&self) {}
}

// gcd(a, b, c) = gcd(a, gcd(b, c)) = gcd(gcd(a, b), c) = gcd(gcd(a, c), b).

// implementación naive del algoritmo de Euclides
fn maximo_comun_divisor_dupla(numero_1: &Entero, numero_2: &Entero) -> Entero {
    match numero_1.cmp(&numero_2) {
        Ordering::Equal => *numero_1,
        Ordering::Greater => maximo_comun_divisor_dupla(&(numero_1 - numero_2), numero_2),
        Ordering::Less => maximo_comun_divisor_dupla(numero_1, &(numero_2 - numero_1)),
    }
}

fn divisores_entero_positivo() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_correctamente_mcd() {
        let test_cases: [[Entero; 3]; 4] =
            [[36, 60, 12], [154, 374, 22], [2377, 1284, 1], [57, 87, 3]];
        for case in test_cases {
            assert_eq!(case[2], maximo_comun_divisor_dupla(&case[0], &case[1]))
        }
    }
}
