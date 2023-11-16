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
use std::io::{self, stdout, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
// Biblioteca para la generación de valores aleatorios
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    let mut game = Game::default();
    let mut input_buffer = String::new();
    let (tx, rx) = mpsc::channel();
    let mut stdout = stdout();

    // Contador de tiempo
    let _ = timer(rx);

    loop {
        // make question
        game.make_question();
        let _ = stdout.write_fmt(format_args!("{game}\n"));
        stdout.flush().unwrap();
        // activate counter
        let _ = tx.send(true);
        // wait and verify answer
        std::io::stdin()
            .read_line(&mut input_buffer)
            .expect("Unexpected error while reading input.");
        let game_over = match input_buffer.trim().parse() {
            Ok(answer) => !game.check_answer(answer),
            Err(_) => true,
        };

        // Check if the game continues
        if game_over {
            println!("Game Over\nScore: {}", game.get_score());
            std::process::exit(0);
        }
        input_buffer.clear();
        let _ = tx.send(false);
    }
}

fn timer(rx: mpsc::Receiver<bool>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut run_clock = false;
        let mut ticks: u8 = 0;
        loop {
            if run_clock {
                thread::sleep(Duration::from_millis(1000));
                ticks += 1;
            }
            match rx.try_recv() {
                Ok(true) => {
                    run_clock = true;
                    ticks = 0;
                }
                Ok(false) => run_clock = false,
                Err(TryRecvError::Empty) => {
                    continue;
                }
                Err(TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
    })
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
    pub fn make_question(&mut self) {
        // llama generate_numbers para obtener el par de operando
        // = U+003D Equals Sign
        let (first_digits, second_digits) = self.generate_operands_digits();
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

    fn generate_operands_digits(&self) -> (usize, usize) {
        let total_digits: usize = self.difficulty_level + 1;
        let second_operand: usize = total_digits / 2;
        let first_operand: usize = total_digits - second_operand;
        (first_operand, second_operand)
    }

    // Obtiene un valor de operador aleatorio entre en el intervalo [0, 9*D],
    // donde D es un numero de la forma 1111... conteniendo digit_number dígitos.
    fn sample_operand(digit_number: usize) -> isize {
        let upper_bound = "9".repeat(digit_number).parse().unwrap();
        // para permitir el cero es necesario agregar un control cuando el divisor es 0.
        StdRng::from_entropy().gen_range(1..=upper_bound)
    }

    // Escoge aleatoriamente la operación a realizar
    fn sample_operator() -> Operator {
        StdRng::from_entropy().sample(Standard)
    }

    pub fn check_answer(&mut self, answer: isize) -> bool {
        if self.answer == answer {
            self.score += 1;
            self.update();
            return true;
        }
        false
    }

    // Actualiza los contadores internos
    fn update(&mut self) {
        if self.score % 5 == 0 {
            self.difficulty_level += 1;
        }
    }

    pub fn get_score(&self) -> &usize {
        &self.score
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
        assert_eq!((1, 1), game.generate_operands_digits());
        game.difficulty_level = 2;
        assert_eq!((2, 1), game.generate_operands_digits());
        game.difficulty_level = 3;
        assert_eq!((2, 2), game.generate_operands_digits());
        game.difficulty_level = 4;
        assert_eq!((3, 2), game.generate_operands_digits());
        game.difficulty_level = 12;
        assert_eq!((7, 6), game.generate_operands_digits());
        game.difficulty_level = 13;
        assert_eq!((7, 7), game.generate_operands_digits());
        game.difficulty_level = 81;
        assert_eq!((41, 41), game.generate_operands_digits());
    }
}
