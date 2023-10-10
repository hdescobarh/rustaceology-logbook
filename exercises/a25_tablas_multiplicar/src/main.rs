/*
Reto #40: Tabla de multiplicar
Crea un programa que sea capaz de solicitarte un número y se
encargue de imprimir su tabla de multiplicar entre el 1 y el 10.
- Debe visualizarse qué operación se realiza y su resultado.
  Ej: 1 x 1 = 1
      1 x 2 = 2
      1 x 3 = 3
      ...
 */

const PROMPT_MSG: &str = "Ingrese el número del que desea ver la tabla de multiplicar:";
const PARSE_ERR_MSG: &str = "El valor ingresado no es valido; Debe ser número entero no negativo.";
const READ_ERR_MSG: &str = "Error inesperado al leer la linea.";
type Number = u128;

fn main() {
    let mut input_buffer = String::new();
    loop {
        println!("{}", PROMPT_MSG);
        std::io::stdin()
            .read_line(&mut input_buffer)
            .expect(READ_ERR_MSG);
        match input_buffer.trim().parse::<Number>() {
            Ok(number) => {
                println!("{}", MultiplicationTable::format_from_multiplicand(&number));
                std::process::exit(0);
            }
            Err(_) => {
                println!("{}", PARSE_ERR_MSG);
                input_buffer.clear();
                continue;
            }
        }
    }
}

pub struct MultiplicationTable {}

impl MultiplicationTable {
    pub fn format_from_multiplicand(multiplicand: &Number) -> String {
        let mut text: String = String::new();
        for multiplier in 1..11 {
            let product = multiplicand * multiplier;
            text.push_str(&format!("{multiplicand} x {multiplier} = {product}\n"));
        }
        text
    }
}
