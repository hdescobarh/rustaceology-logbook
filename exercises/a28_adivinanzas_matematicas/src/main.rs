// author: Hans D. Escobar H. (hdescobarh)

/* Reto Mouredev #44: Adivinanzas matemáticas
Crea un juego interactivo por terminal en el que tendrás que adivinar
el resultado de diferentes operaciones matemáticas aleatorias (suma, resta,
multiplicación o división de dos números enteros).

- Tendrás 3 segundos para responder correctamente.
- El juego finaliza si no se logra responder en ese tiempo.
- Al finalizar el juego debes mostrar cuántos cálculos has acertado.
- Cada 5 aciertos debes aumentar en uno el posible número de cifras
de la operación (cada vez en un operando):

    - Preguntas 1 a 5: X (entre 0 y 9) operación Y (entre 0 y 9)
    - Preguntas 6 a 10: XX (entre 0 y 99) operación Y (entre 0 y 9)
    - Preguntas 11 a 15: XX operación YY
    - Preguntas 16 a 20: XXX (entre 0 y 999) operación YY
    - ...

 */

// Biblioteca para la generación de valores aleatorios
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    println!("Hello, world!");
}

pub enum Operator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

// Distribución de probabilidad para generar operadores aleatorios
impl Distribution<Operator> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operator {
        let n: u8 = rng.gen_range(0..=4);
        match n {
            0 => Operator::Addition,
            1 => Operator::Subtraction,
            2 => Operator::Division,
            _ => Operator::Multiplication,
        }
    }
}

pub struct Game {
    score: usize,
    difficulty_level: usize,
    question: (usize, usize, Operator),
    answer: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            score: 0,
            difficulty_level: 1,
            question: (0, 0, Operator::Addition),
            answer: 0,
        }
    }
}

impl Game {
    // construye la operación y almacena la respuesta valida
    fn make_operation(&mut self) {
        // llama generate_numbers para obtener el par de operando
        // = U+003D Equals Sign
        todo!()
    }

    // Genera una dupla de números aleatorios
    fn get_random_operands(&self) -> (usize, usize) {
        todo!()
    }

    fn get_operands_digits(&self) -> (usize, usize) {
        let total_digits: usize = self.difficulty_level + 1;
        let second_operand: usize = total_digits / 2;
        let first_operand: usize = total_digits - second_operand;
        (first_operand, second_operand)
    }

    // Escoge aleatoriamente la operación a realizar
    fn get_random_operator() -> Operator {
        let operator: Operator = StdRng::from_entropy().sample(Standard);
        operator
    }

    // Actualiza los contadores internos
    fn update(&mut self, correct_answer: bool) {
        if self.difficulty_level % 5 == 0 {
            self.difficulty_level += 1;
        }

        if correct_answer {
            self.score += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn operand_digits_works() {
        let mut game = Game::default();
        assert_eq!((1, 1), game.get_operands_digits());
        game.difficulty_level = 2;
        assert_eq!((2, 1), game.get_operands_digits());
        game.difficulty_level = 3;
        assert_eq!((2, 2), game.get_operands_digits());
        game.difficulty_level = 4;
        assert_eq!((3, 2), game.get_operands_digits());
        game.difficulty_level = 12;
        assert_eq!((7, 6), game.get_operands_digits());
        game.difficulty_level = 13;
        assert_eq!((7, 7), game.get_operands_digits());
        game.difficulty_level = 81;
        assert_eq!((41, 41), game.get_operands_digits());
    }
}
