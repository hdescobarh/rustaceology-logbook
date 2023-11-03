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

use rand::distributions::{Bernoulli, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// Máximo valor permitido en el modelo
const MAX_ALLOWED_TEMPERATURE: f64 = 90.0;
// Mínimo valor permitido en el modelo
const MIN_ALLOWED_TEMPERATURE: f64 = -110.0;

pub struct Simulation {
    total_days: usize,
    sample: Vec<Forecast>,
}

impl Simulation {
    pub fn run(days: usize, init_temperature: f64, init_rain_probability: f64) -> Self {
        let mut sample: Vec<Forecast> = Vec::with_capacity(days);
        sample.push(Forecast::new(
            false,
            init_temperature,
            init_rain_probability,
        ));
        let mut simulation = Self {
            total_days: days,
            sample,
        };
        simulation.sample();
        simulation
    }

    fn sample(&mut self) {
        while self.sample.len() < self.total_days {
            let lastest_forecast = &self.sample[self.sample.len() - 1];
            let next_forecast = lastest_forecast.next_day();
            self.sample.push(next_forecast);
        }
    }

    fn get_stats(&self) -> (usize, f64, f64) {
        let days_with_rain = self
            .sample
            .iter()
            .filter(|forecast| forecast.rained)
            .count();
        let max_temperature = self
            .sample
            .iter()
            .map(|forecast| forecast.temperature)
            .reduce(f64::max)
            .unwrap();
        let min_temperature = self
            .sample
            .iter()
            .map(|forecast| forecast.temperature)
            .reduce(f64::min)
            .unwrap();
        (days_with_rain, max_temperature, min_temperature)
    }

    pub fn report(&self) -> String {
        let mut msg = "Day\tTemperature\tRain\n".to_string();
        for (day, forecast) in self.sample.iter().enumerate() {
            let temperature = forecast.temperature;
            let rained = forecast.rained;
            msg.push_str(&format!("{day}\t{temperature}\t{rained}\n"));
        }
        let (days_with_rain, max_temperature, min_temperature) = self.get_stats();
        msg.push_str(&format!(
            "\
            - Days with rain: {days_with_rain}\n\
            - Max. temperature (ºC): {max_temperature}\n\
            - Min. temperature (ºC): {min_temperature}\n"
        ));
        msg
    }
}

/// Representa las predicciones de un día
struct Forecast {
    /// Temperatura en Celsius
    temperature: f64,
    /// Probabilidad de lluvia
    rain_probability: f64,
    /// Sí efectivamente llovió es `true`
    rained: bool,
}

impl Forecast {
    fn new(rained: bool, temperature: f64, rain_probability: f64) -> Self {
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
    fn next_day(&self) -> Self {
        // Temperatura
        let next_temperature: f64 = self.temperature_control();
        // Probabilidad de lluvia
        let next_rain_probability: f64 = self.rain_probability_control();
        // Resultado efectivo
        let next_rained: bool = Self::rain_get_random(next_rain_probability);

        Self::new(next_rained, next_temperature, next_rain_probability)
    }

    // Controla como cambia la temperatura del día t+1 según sí llovió en el día t
    fn temperature_control(&self) -> f64 {
        let next_temperature: f64;
        if self.rained {
            next_temperature = self.temperature - 1.0;
        } else {
            // Genera un número aleatorio entre [0, 1) a partir de una distribución uniforme
            let val: f64 = StdRng::from_entropy().sample(Standard);
            // La P(cambio ±2ºC) = 0.1, asumiendo que P(suba 2 | cambió) = P(disminuya 2 | cambió) = 0.5,
            // se tiene que P(suba 2) = P(disminuya 2) = 0.05
            if (0.0..0.05).contains(&val) {
                next_temperature = self.temperature + 2.0;
            } else if (0.05..0.1).contains(&val) {
                next_temperature = self.temperature - 2.0;
            } else if (0.1..1.0).contains(&val) {
                next_temperature = self.temperature;
            } else {
                panic!("Unexpected error. Value out of [0, 1)")
            }
        };
        next_temperature
    }

    // Controla como cambia la probabilidad de lluvia del día t+1 según la temperatura del día t
    fn rain_probability_control(&self) -> f64 {
        let next_rain_probability: f64;

        if self.temperature > 25.0 {
            next_rain_probability = f64::min(self.rain_probability + 0.2, 1.0);
        } else if self.temperature < 5.0 {
            next_rain_probability = f64::max(self.rain_probability - 0.2, 0.0)
        } else {
            next_rain_probability = self.rain_probability
        }
        next_rain_probability
    }

    // Genera un resultado aleatorio de lluvia para el día siguiente
    fn rain_get_random(next_rain_probability: f64) -> bool {
        StdRng::from_entropy().sample(Bernoulli::new(next_rain_probability).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
