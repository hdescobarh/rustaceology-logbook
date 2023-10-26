// author: Hans D. Escobar H. (hdescobarh)
/*
Reto Mouredev #42: Punto de encuentro
 Crea una función que calcule el punto de encuentro de dos objetos en movimiento
 en dos dimensiones.
 - Cada objeto está compuesto por una coordenada xy y una velocidad de desplazamiento
   (vector de desplazamiento) por unidad de tiempo (también en formato xy).
 - La función recibirá las coordenadas de inicio de ambos objetos y sus velocidades.
 - La función calculará y mostrará el punto en el que se encuentran y el tiempo que tardarn en lograrlo.
 - La función debe tener en cuenta que los objetos pueden no llegar a encontrarse.
 */

/*
- El ejercicio no especifica el tipo de movimiento. Asumiré que se trata de
Movimiento Rectilíneo Uniforme; es decir, el vector aceleración es nulo para todo tiempo t.

- En el sentido estricto los vectores velocidad y desplazamiento son dos conceptos diferentes,
el vector velocidad
 */

use std::iter::FromIterator;
use std::ops::{Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Mul<Vector2D> for Vector2D {
    type Output = f64;
    fn mul(self, rhs: Vector2D) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl From<&[f64; 2]> for Vector2D {
    fn from(value: &[f64; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct Object2D {
    location: Vector2D,
    velocity: Vector2D,
}

impl Object2D {
    // constructor
    pub fn new(location: &[f64; 2], velocity: &[f64; 2]) -> Self {
        Self {
            location: Vector2D::from(location),
            velocity: Vector2D::from(velocity),
        }
    }
}

impl UniformLinearMotion for Object2D {
    /*
    La lógica es que sí los dos objetos se encuentran, entonces necesariamente existe un tiempo 𝘵,
    tal que la distancia euclídea entre las posiciones 𝗽 y 𝗽' del primer y segundo objeto,
    respectivamente, es cero:
    (1)    ∃𝘵‖𝗽(𝘵)-𝗽'(𝘵)‖ = ‖Δ𝗽(𝘵)‖ = 0
    En movimiento linear uniforme, la velocidad 𝐯 es constante para todo tiempo 𝘵; por lo tanto:
    (2)    𝗽(𝘵) = 𝗽₀ + 𝘵𝐯
    Por propiedades del producto punto, (1) y (2):
    (3)    ⟨Δ𝗽(𝘵), Δ𝗽(𝘵)⟩ = 𝘵² ⟨Δ𝐯,Δ𝐯⟩ + 𝘵 2⟨Δ𝐯,Δ𝗽₀⟩ + ⟨Δ𝗽₀,Δ𝗽₀⟩ = 0
    */
    fn ulm_collision_time(&self, other: &Self) -> Option<f64> {
        // obtenemos Δ𝗽₀ y Δ𝐯
        let delta_initial_position: Vector2D = self.location - other.location;
        let delta_velocity: Vector2D = self.velocity - other.velocity;
        // Es un polinomio de la forma ax² + bx + c, donde a=⟨Δ𝐯,Δ𝐯⟩, b=2⟨Δ𝐯,Δ𝗽₀⟩, c=⟨Δ𝗽₀,Δ𝗽₀⟩
        let a: f64 = delta_velocity * delta_velocity;
        let b: f64 = delta_velocity * delta_initial_position * 2.0;
        let c: f64 = delta_initial_position * delta_initial_position;
        // sí ambos objetos llevan la misma velocidad, a=0 y b=0. El problema es lineal 0x + c = 0
        if a == 0.0_f64 {
            // sí c=0. Hay infinitas soluciones: 0x = 0
            if c.abs() < f64::EPSILON {
                return Some(0.0);
            // sí por el contrario, c≠0, no hay solución: 0x = c
            } else {
                return None;
            }
        }
        // Cuando a≠0 es un problema cuadrático y aplicamos
        // la formula cuadrática: ( -b +- sqrt(b² - 4 ac) ) / (2a),
        // sí tiene solución en los reales, necesariamente b² - 4ac ≧ 0
        let discriminant = b.powi(2) - (4.0 * a * c);
        if discriminant < 0.0_f64 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let solution_1 = (-b - sqrt_discriminant) / (2.0 * a);
        let solution_2 = (-b + sqrt_discriminant) / (2.0 * a);
        // es movimiento rectilíneo uniforme en un espacio euclídea; por lo tanto,
        // a lo sumo existe solo una solución
        if solution_1 >= 0.0_f64 {
            Some(solution_1)
        } else if solution_2 > 0.0_f64 {
            Some(solution_2)
        } else {
            None
        }
    }
}

trait UniformLinearMotion {
    ///
    fn ulm_collision_time(&self, other: &Self) -> Option<f64>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulm_collision_time_by_axis() {
        // collision with static object
        let test_cases = [
            (
                [[9.0, 0.0], [60.0, 0.0], [9.0, 0.0], [60.0, 0.0]],
                Some(0.0),
            ), // same origin and velocity
            (
                [[15.8, 0.0], [80.3, 0.0], [15.8, 0.0], [-50.0, 0.0]],
                Some(0.0),
            ), // same origin, different velocity
            ([[17.0, 0.0], [123.5, 0.0], [1.8, 0.0], [123.5, 0.0]], None), // different origin, same velocity
            // from here, different origin and velocity
            ([[0.0, 0.0], [0.5, 0.0], [6.0, 0.0], [0.0, 0.0]], Some(12.0)),
            (
                [[-2.5, 0.0], [2.406, 0.0], [4.5, 0.0], [2.1, 0.0]],
                Some(22.8758169),
            ),
            ([[98.0, 0.0], [30.0, 0.0], [15.0, 0.0], [-42.0, 0.0]], None),
            (
                [[12.0, 0.0], [2.7, 0.0], [179.0, 0.0], [-3.1, 0.0]],
                Some(28.7931034),
            ),
        ];
        for ([loc1, vel1, loc2, vel2], expected) in &test_cases {
            let object_1 = Object2D::new(loc1, vel1);
            let object_2 = Object2D::new(loc2, vel2);
            let solution = object_1.ulm_collision_time(&object_2);
            if expected.is_some() && solution.is_some() {
                assert!(
                    (expected.unwrap() - solution.unwrap()).abs() < 1E-6,
                    "Expected {:?}, obtained {:?}",
                    expected,
                    solution
                )
            } else {
                assert_eq!(*expected, solution)
            };
        }
    }
}
