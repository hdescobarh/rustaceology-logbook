// author: Hans D. Escobar H. (hdescobarh)

/*
Crea las funciones capaces de transformar colores HEX
a RGB y viceversa.
Ejemplos:
RGB a HEX: r: 0, g: 0, b: 0 -> #000000
HEX a RGB: hex: #000000 -> (r: 0, g: 0, b: 0)
*/

fn main() {
    let my_str = "#0F00FA";
}

/***
 * Bajo el esquema True Color (24 bits) los colores se representan como una tripleta (Red, Green, Blue),
 * La notación RGB o HEX cambia la forma en que se representan los valores:
 * - RGB, como una tupla de decimales de 8 bits (0-255)
 * - HEX, hexádecimales de 8 bits escritos en dos dígitos (00-FF), concatenados y con trailing "#". Preserva el orden RGB.
 */
struct TrueColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl TrueColor {
    /// Crea un TrueColor desde una cadena de texto. Solo acepta HEX en formato de 6 valores (e.g. "#F1F2F3")
    pub fn from_hex(value: String) -> Self {
        /* Transforma la String en un iterador de caracteres y
        valida que tenga "#" como trailing character */
        let mut chars = value.chars();
        if chars.next().unwrap() != '#' {
            panic!("Invalid")
        };

        /* Toma los demas caracteres del iterador, los separa parejas,
        y cada pareja trata de interpretarla como un literal en base 16 (hexadecimal)*/
        let valores = chars
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunck| u8::from_str_radix(&String::from_iter(chunck), 16).unwrap())
            .collect::<Vec<u8>>();
        if valores.len() != 3 {
            panic!("Invalid")
        };

        TrueColor {
            red: valores[0],
            green: valores[1],
            blue: valores[2],
        }
    }

    /// Genera una String con el color en formato HEX
    fn get_hex(&self) -> String {
        format!("#{:X}{:X}{:X}", self.red, self.green, self.blue)
    }

    /// Crea un TrueColor desde una tripleta de valores de 8-bits (0-255)
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        // Los valores ya son unsigned de 8 bits. No hace falta validaciones adicionales
        TrueColor { red, green, blue }
    }

    /// Genera una String con el color en formato RGB
    fn get_rgb(&self) -> String {
        format!("(r: {}, g: {}, b: {})", self.red, self.green, self.blue)
    }

    /// Traduce de una notación RGB valida a un HEX
    pub fn rgb2hex(red: u8, green: u8, blue: u8) -> String {
        let true_color = Self::from_rgb(red, green, blue);
        true_color.get_hex()
    }
    /// Traduce de una notación HEX valida a RGB
    pub fn hex2rgb(value: String) -> String {
        let true_color = Self::from_hex(value);
        true_color.get_rgb()
    }
}
