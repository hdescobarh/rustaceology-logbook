// author: Hans D. Escobar H. (hdescobarh)

/*
 * Crea una función que encuentre todas las combinaciones de los números
 * de una lista que suman el valor objetivo.
 * - La función recibirá una lista de números enteros positivos
 *   y un valor objetivo.
 * - Para obtener las combinaciones sólo se puede usar
 *   una vez cada elemento de la lista (pero pueden existir
 *   elementos repetidos en ella).
 * - Ejemplo: Lista = [1, 5, 3, 2],  Objetivo = 6
 *   Soluciones: [1, 5] y [1, 3, 2] (ambas combinaciones suman 6)
 *   (Si no existen combinaciones, retornar una lista vacía)
 */

/*  Rust no cuenta con funciones para generar combinaciones en su
biblioteca estándar (https://doc.rust-lang.org/std/).
La idea es no usar bibliotecas externas, así que implementaré un algoritmo de generar combinaciones,
pero modificado para que solo busque las combinaciones menores o iguales al Objetivo.
 */

/// Almacena una posible combinación de una secuencia:
/// valores: la combinación formada
/// pendientes: valores de la secuencia que no hacen parte de los valores de la combinación
/// suma: la suma de los valores de la combinación
#[derive(Clone)]
pub struct Combinacion {
    valores: Vec<usize>,
    pendientes: Vec<usize>,
    suma: usize,
}

impl Combinacion {
    /// Desde una secuencia genera todas las posibles combinaciones que satisfacen
    /// que su suma es igual al valor objetivo
    pub fn desde_secuencia(secuencia: &[usize], objetivo: &usize) -> Vec<Vec<usize>> {
        // genera la raíz del arbol de búsqueda
        let nodo_inicial = Self {
            valores: Vec::new(),
            pendientes: secuencia.to_vec(),
            suma: 0,
        };
        // La función asociada "generar_combinaciones" de forma recursiva genera combinaciones que cumplen la condición
        let mut buffer: Vec<Combinacion> = Vec::new();
        Combinacion::generar_combinaciones(&mut buffer, vec![nodo_inicial], objetivo);
        // Extrae los valores de cada combinación
        buffer
            .into_iter()
            .map(|combinacion| combinacion.valores)
            .collect::<Vec<Vec<usize>>>()
    }

    fn generar_combinaciones(
        buffer: &mut Vec<Combinacion>,
        base: Vec<Combinacion>,
        objetivo: &usize,
    ) {
        for combinacion in base {
            if let Some(nueva_base) = combinacion.derivar_desde_pendientes(objetivo) {
                Combinacion::generar_combinaciones(buffer, nueva_base, objetivo);
            }
            if combinacion.suma == *objetivo {
                buffer.push(combinacion);
            }
        }
    }

    /// A partir de una combinación de n valores y k > 0 pendientes genera todas las combinaciones
    /// validas de n+1 valores que derivan de esta.
    /// Sí no se puede derivar mas combinaciones, retorna None.
    fn derivar_desde_pendientes(&self, objetivo: &usize) -> Option<Vec<Combinacion>> {
        if self.pendientes.is_empty() {
            return None;
        }
        let mut nuevas_combinaciones: Vec<Combinacion> = Vec::with_capacity(self.pendientes.len());
        for posicion in 0..self.pendientes.len() {
            // define la suma y sí esta es mayor al objetivo, no genera una nueva combinación
            let suma: usize = self.suma + self.pendientes[posicion];
            if suma > *objetivo {
                continue;
            }
            // Genera una nueva combinación, extendiendo la original con el valor del indice i de pendientes.
            // Los pendiente de la nueva combinación corresponde
            // a los indices i+1..n de los pendientes de la combinación original. Este paso es el que garantiza que no se repitan posiciones
            let mut nueva = self.clone();
            nueva.pendientes = nueva.pendientes.split_off(posicion);
            nueva.suma = suma;
            nueva.valores.push(nueva.pendientes.swap_remove(0));
            nuevas_combinaciones.push(nueva)
        }
        Some(nuevas_combinaciones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_resultado_suma_objetivo() {
        let objetivo = 10;
        let secuencia = [11, 5, 3, 2, 7, 20, 1, 5, 4, 1, 3, 9, 11, 20, 13, 17];
        let resultado = Combinacion::desde_secuencia(&secuencia, &objetivo);
        for combinacion in resultado {
            assert_eq!(objetivo, combinacion.iter().sum());
        }
    }

    #[test]
    fn caso_ejemplo() {
        let secuencia = [1, 5, 3, 2];
        let objetivo: usize = 6;
        let esperado = vec![vec![1, 2, 3], vec![1, 5]];
        let mut resultado = Combinacion::desde_secuencia(&secuencia, &objetivo);
        resultado.sort();
        assert_eq!(esperado, resultado);
    }

    #[test]
    fn caso_ejemplo_extendido() {
        let secuencia = [11, 5, 3, 2, 7, 20, 1, 5, 4];
        let objetivo: usize = 6;
        let esperado = vec![vec![1, 5], vec![2, 4], vec![3, 2, 1], vec![5, 1]];
        let mut resultado = Combinacion::desde_secuencia(&secuencia, &objetivo);
        resultado.sort();
        assert_eq!(esperado, resultado);
    }
}
