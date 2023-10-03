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

pub struct TriplePitagorico {
    cateto_a: usize,
    cateto_b: usize,
    hipotenusa: usize,
}

impl TriplePitagorico {
    // crea un Tripe Pitagórico a partir de un numero, el cual se asume que es el máximo del trio y , por lo tanto
    // la hipotenusa
    pub fn desde_numero_maximo(numero: &usize) {}

    // devuelve una lista de los divisores comunes del triple diferentes de 1
    // si el máximo común divisor es uno, retorna None
    fn encontrar_divisores_comunes(&self) {
        // primero encuentra el máximo común divisor para los tres números. Se aplica la propiedad:
        // gcd(a, b, c) = gcd(a, gcd(b, c)) = gcd(gcd(a, b), c) = gcd(gcd(a, c), b).
        let triple_mcd = maximo_comun_divisor(
            &self.hipotenusa,
            &maximo_comun_divisor(&self.cateto_a, &self.cateto_b),
        );
        // seguido, se encuentra todos los divisores del mcd.
        // Estos serán todos los divisores posibles de triple
    }

    //
    fn generar_triples_menores(&self) {}
}

// encuentra el máximo común divisor de un par de números enteros positivos
// implementación naive del algoritmo de Euclides
fn maximo_comun_divisor(numero_1: &usize, numero_2: &usize) -> usize {
    match numero_1.cmp(&numero_2) {
        Ordering::Equal => *numero_1,
        Ordering::Greater => maximo_comun_divisor(&(numero_1 - numero_2), numero_2),
        Ordering::Less => maximo_comun_divisor(numero_1, &(numero_2 - numero_1)),
    }
}

// encuentra los divisores diferentes de 1 y el mismo número
// implementación naive
fn obtener_divisores(numero: &usize) -> Vec<usize> {
    let mut divisores: Vec<usize> = Vec::with_capacity(*numero);
    for candidato in 2..*numero {
        if (numero % candidato) == 0 {
            divisores.push(candidato)
        }
    }
    divisores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_correctamente_mcd() {
        let test_cases: [[usize; 3]; 4] =
            [[36, 60, 12], [154, 374, 22], [2377, 1284, 1], [57, 87, 3]];
        for case in test_cases {
            assert_eq!(case[2], maximo_comun_divisor(&case[0], &case[1]))
        }
    }

    #[test]
    fn encuentra_divisores_de_un_numero() {
        let test_cases = [
            (7, vec![]),
            (12, vec![2, 3, 4, 6]),
            (25, vec![5]),
            (42, vec![2, 3, 6, 7, 14, 21]),
            (97, vec![]),
        ];

        for case in test_cases {
            assert_eq!(case.1, obtener_divisores(&case.0));
        }
    }
}
