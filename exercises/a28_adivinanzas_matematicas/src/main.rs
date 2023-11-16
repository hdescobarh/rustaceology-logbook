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
use std::num::ParseIntError;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

// Biblioteca para la generación de valores aleatorios
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Indica cuanto tiempo (en ms) dura cada tick del juego
pub const TICK_LENGTH: u64 = 1000;
/// Indica los conteos máximos por pregunta. Multiplicado por TICK_LENGTH resulta en el tiempo en ms por pregunta.
pub const MAX_TICKS_PER_QUESTION: usize = 3;

fn main() {
    let mut game = Game::default();
    // initial question
    game.make_question();
    println!("{game}");

    let stdin_rx = stdin_channel();
    let mut ticks: usize = 0;
    loop {
        ticks += 1;
        //println!("{ticks}");
        thread::sleep(Duration::from_millis(TICK_LENGTH));

        let game_over: bool = if ticks >= MAX_TICKS_PER_QUESTION {
            true
        } else {
            match stdin_rx.try_recv() {
                Ok(Ok(answer)) => !game.check_answer(answer),
                Ok(Err(_)) => true,
                Err(TryRecvError::Empty) => continue,
                Err(TryRecvError::Disconnected) => break,
            }
        };

        if game_over {
            println!("Game Over\nScore: {}", game.get_score());
            std::process::exit(0);
        }

        game.make_question();
        println!("{game}");
        ticks = 0;
    }
}

fn stdin_channel() -> Receiver<Result<isize, ParseIntError>> {
    let (tx, rx) = channel();
    thread::spawn(move || loop {
        let mut input_buffer = String::new();
        std::io::stdin()
            .read_line(&mut input_buffer)
            .expect("Unexpected error while reading input.");
        tx.send(input_buffer.trim().parse::<isize>()).unwrap();
    });
    rx
}

/// Operadores validos a emplear en el juego
pub enum Operator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
}

/// Distribución de probabilidad para generar operadores de forma aleatoria
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

/// Implementa el juego y sus normas
pub struct Game {
    /// Marcador de puntos. Otorga un punto por cada respuesta correcta. Cada 5 puntos incrementa la dificultad en uno.
    score: usize,
    /// Indica la dificultad del juego. El número de dígitos totales de cada pregunta es la dificultad + 1.
    difficulty_level: usize,
    /// Contiene el primer y segundo operando y el operador.
    question: (isize, isize, Operator),
    /// Contiene el resultado correcto de la operación.
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
    /// Genera una operación valida y su respuesta correcta.
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

    // Determina los dígitos del primer y segunda operando
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

    /// Verifica si la respuesta ingresada es correcta. Retorna true sí es correcta.
    ///
    /// # Argumentos:
    /// * `answer` - respuesta ingresada
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

    /// Presenta la tabla de puntuación
    pub fn get_score(&self) -> &usize {
        &self.score
    }
}

// Implementación de la vista para Game
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
