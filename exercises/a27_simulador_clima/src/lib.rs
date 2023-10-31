// Author: Hans D. Escobar H. (hdescobarh)

/*
 * Reto #43 Mouredev 2023
 * Crea una función que simule las condiciones climáticas (temperatura y probabilidad de lluvia)
 * de un lugar ficticio al pasar un número concreto de días según estas reglas:
 * - La temperatura inicial y el % de probabilidad de lluvia lo define el usuario.
 * - Cada día que pasa:
 *   - 10% de posibilidades de que la temperatura aumente o disminuya 2 grados.
 *   - Si la temperatura supera los 25 grados, la probabilidad de lluvia al día
 *     siguiente aumenta en un 20%.
 *   - Si la temperatura baja de 5 grados, la probabilidad de lluvia al día
 *     siguiente disminuya en un 20%.
 *   - Si llueve (100%), la temperatura del día siguiente disminuye en 1 grado.
 * - La función recibe el número de días de la predicción y muestra la temperatura
 *   y si llueve durante todos esos días.
 * - También mostrará la temperatura máxima y mínima de ese periodo y cuántos días va a llover.
 */

use rand::distributions::Standard;
use rand::prelude::*;

pub const FLOAT_TOLERANCE: f64 = 1E-10;
/// Máximo valor permitido en el modelo
pub const MAX_ALLOWED_TEMPERATURE: f64 = 90.0;
/// Mínimo valor permitido en el modelo
pub const MIN_ALLOWED_TEMPERATURE: f64 = -110.0;

/// Representa las predicciones de un día
pub struct Forecast {
    /// Temperatura en Celsius
    temperature: f64,
    /// Probabilidad de lluvia
    rain_probability: f64,
    /// Sí efectivamente llovió es `true`
    rained: bool,
}

impl Forecast {
    pub fn new(rained: bool, temperature: f64, rain_probability: f64) -> Self {
        if !(0.0..=1.0).contains(&rain_probability) {
            panic!("Probability must be in the close interval [0, 1].")
        }

        if !(MIN_ALLOWED_TEMPERATURE..=MAX_ALLOWED_TEMPERATURE).contains(&temperature) {
            panic!("Temperature out of bounds.")
        }
        Self {
            rained,
            temperature,
            rain_probability,
        }
    }

    // Genera predicciones para el día siguiente
    pub fn next_day(&self) -> Self {
        // Temperatura

        // Probabilidad de lluvia

        // Resultado efectivo

        todo!()
    }

    // Controla como cambia la temperatura del día t+1 según sí llovió en el día t
    fn temperature_control(&self) -> f64 {
        let next_temperature: f64;
        if self.rained {
            next_temperature = self.temperature - 1.0;
        } else {
            // La P(cambio ±2ºC) = 0.1, asumiendo que P(suba 2 | cambió) = P(disminuya 2 | cambió),
            // se tiene que P(suba 2) = P(disminuya 2) = 0.05
        };

        todo!()
    }

    // Controla como cambia la probabilidad de lluvia del día t+1 según la temperatura del día t
    fn rain_probability_control(&self) -> f64 {
        todo!()
    }

    // Genera un resultado aleatorio para el día siguiente
    fn rain_get_random() -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
