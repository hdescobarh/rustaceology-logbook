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
use std::cmp::min;

/// Representa un triple pitagórico.
///
/// Un triple pitagórico se define como un conjunto de tres números {a, b, c}
/// que satisfacen a² + b² = c², tal que a,b,c ∈ ℤ⁺.
/// Un triple cumple que a > b > c, por lo que se puede anotar como (a,b,c)
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TriplePitagorico {
    cateto_menor: usize,
    cateto_mayor: usize,
    hipotenusa: usize,
}

impl TriplePitagorico {
    /// Genera todos los triples pitagóricos que satisfacen que su máximo valor es menor o igual al número especificado.
    ///
    /// Equivalente a, dada una constante r ∈ ℤ⁺, encontrar todos pares (a,b) tales que
    /// a² + b² = c², c² ≤ r²
    /// ⇒ a² ≤ r² - b², a < b < r
    /// ⇒ a² < min(b, r² - b² + 1)
    ///
    /// # Argumentos:
    /// * `numero` - indica el máximo valor que puede aparecer en los triples pitagóricos generados.
    pub fn desde_numero_maximo(numero: &usize) -> Option<Vec<TriplePitagorico>> {
        let mut triples: Vec<Self> = Vec::new();
        for b in 3..*numero {
            for a in 2..b {
                if let Some(c) = raiz_cuadrada_perfecta(&(a.pow(2) + b.pow(2))) {
                    if c > *numero {
                        continue;
                    }

                    triples.push(Self {
                        cateto_menor: a,
                        cateto_mayor: b,
                        hipotenusa: c,
                    })
                };
            }
        }
        if triples.is_empty() {
            None
        } else {
            Some(triples)
        }
    }
}

fn raiz_cuadrada_perfecta(numero: &usize) -> Option<usize> {
    let raiz_entera = (*numero as f64).sqrt().floor() as usize;
    if raiz_entera.pow(2) == *numero {
        Some(raiz_entera)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn no_hay_triple_para_valor() {
        let resultado = TriplePitagorico::desde_numero_maximo(&4);
        assert_eq!(None, resultado);
    }
    #[test]
    fn genera_triple_y_multiplo() {
        let resultado: HashSet<TriplePitagorico> = TriplePitagorico::desde_numero_maximo(&10)
            .unwrap()
            .into_iter()
            .collect();
        let esperado: HashSet<TriplePitagorico> = [[3, 4, 5], [6, 8, 10]]
            .into_iter()
            .map(|tripla| TriplePitagorico {
                cateto_menor: tripla[0],
                cateto_mayor: tripla[1],
                hipotenusa: tripla[2],
            })
            .collect();
        assert_eq!(esperado, resultado);
    }

    #[test]
    fn genera_triples_menores_o_iguales() {
        let resultado: HashSet<TriplePitagorico> = TriplePitagorico::desde_numero_maximo(&25)
            .unwrap()
            .into_iter()
            .collect();
        let esperado: HashSet<TriplePitagorico> = [
            [3, 4, 5],
            [6, 8, 10],
            [5, 12, 13],
            [9, 12, 15],
            [8, 15, 17],
            [12, 16, 20],
            [15, 20, 25],
            [7, 24, 25],
        ]
        .into_iter()
        .map(|tripla| TriplePitagorico {
            cateto_menor: tripla[0],
            cateto_mayor: tripla[1],
            hipotenusa: tripla[2],
        })
        .collect();
        assert_eq!(esperado, resultado);
    }
}
