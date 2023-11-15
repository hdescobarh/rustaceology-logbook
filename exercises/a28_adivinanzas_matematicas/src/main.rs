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

use std::fmt::Display;

// Biblioteca para la generación de valores aleatorios
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    let mut game = Game::default();
    game.make_question();
    println!("{game}")
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
    question: (isize, isize, Operator),
    answer: isize,
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
    fn make_question(&mut self) {
        // llama generate_numbers para obtener el par de operando
        // = U+003D Equals Sign
        let (first_digits, second_digits) = self.get_operands_digits();
        let first_operand = Self::sample_operand(first_digits);
        let second_operand = Self::sample_operand(second_digits);

        let operator = Self::sample_operator();
        let answer: isize = match operator {
            Operator::Addition => first_operand + second_operand,
            Operator::Subtraction => first_operand - second_operand,
            Operator::Division => first_operand / second_operand,
            Operator::Multiplication => first_operand * second_operand,
        };
        self.answer = answer;
        self.question = (first_operand, second_operand, operator);
    }

    fn get_operands_digits(&self) -> (usize, usize) {
        let total_digits: usize = self.difficulty_level + 1;
        let second_operand: usize = total_digits / 2;
        let first_operand: usize = total_digits - second_operand;
        (first_operand, second_operand)
    }

    // Obtiene un valor de operador aleatorio entre en el intervalo [0, 9*D],
    // donde D es un numero de la forma 1111... conteniendo digit_number dígitos.
    fn sample_operand(digit_number: usize) -> isize {
        let upper_bound = "9".repeat(digit_number).parse().unwrap();
        StdRng::from_entropy().gen_range(0..=upper_bound)
    }

    // Escoge aleatoriamente la operación a realizar
    fn sample_operator() -> Operator {
        StdRng::from_entropy().sample(Standard)
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

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator_str = match self.question.2 {
            // U+002B Plus Sign
            Operator::Addition => "+",
            // U+2212 Minus Sign
            Operator::Subtraction => "−",
            // U+00F7 Division Sign
            Operator::Division => "÷",
            // U+00D7 Multiplication Sign
            Operator::Multiplication => "×",
        };

        write!(
            f,
            "{} {} {}",
            self.question.0, operator_str, self.question.1
        )
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
