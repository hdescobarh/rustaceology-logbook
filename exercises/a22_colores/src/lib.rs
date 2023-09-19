// author: Hans D. Escobar H. (hdescobarh)

/*
Crea las funciones capaces de transformar colores HEX
a RGB y viceversa.
Ejemplos:
RGB a HEX: r: 0, g: 0, b: 0 -> #000000
HEX a RGB: hex: #000000 -> (r: 0, g: 0, b: 0)
*/

const ERROR_MSG: &str = "Formato invalido";

/***
 * Bajo el esquema True Color (24 bits) los colores se representan como una tripleta (Red, Green, Blue),
 * La notación RGB o HEX cambia la forma en que se representan los valores:
 * - RGB, como una tupla de decimales de 8 bits (0-255)
 * - HEX, hexádecimales de 8 bits escritos en dos dígitos (00-FF), concatenados y con trailing "#". Preserva el orden RGB.
 */
pub struct TrueColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl TrueColor {
    /// Crea un TrueColor desde una cadena de texto en formato Hexadecimal de 6 dígitos (e.g. "#F1F2F3")
    pub fn from_hex(value: String) -> Self {
        /* Transforma la String en un iterador de caracteres y
        valida que tenga "#" como trailing character */
        let mut chars = value.chars();
        if chars.next().unwrap() != '#' {
            panic!("{ERROR_MSG}")
        };

        /* Toma los demas caracteres del iterador, los separa parejas,
        y cada pareja trata de interpretarla como un literal en base 16 (hexadecimal)*/
        let valores = chars
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunck| u8::from_str_radix(&String::from_iter(chunck), 16).expect("{ERROR_MSG}"))
            .collect::<Vec<u8>>();
        if valores.len() != 3 {
            panic!("{ERROR_MSG}")
        };

        TrueColor {
            red: valores[0],
            green: valores[1],
            blue: valores[2],
        }
    }

    /// Genera una String con el color en formato HEX
    fn get_hex(&self) -> String {
        // la notación 0>2 indica que sí el número formateado tiene ancho menor que
        // dos, entonces agrega un 0 alineado a la derecha
        format!("#{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
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

#[cfg(test)]
mod tests {
    use super::*;
    // Crea un alias para tipo existentes
    type TestCase = (&'static str, (u8, u8, u8), &'static str, &'static str);

    // Esto sería mas adecuado en un archivo aparte.
    const VALID_TEST_CASES: [TestCase; 17] = [
        ("Black", (0, 0, 0), "#000000", "(r: 0, g: 0, b: 0)"),
        (
            "White",
            (255, 255, 255),
            "#FFFFFF",
            "(r: 255, g: 255, b: 255)",
        ),
        (
            "MediumSlateBlue",
            (123, 104, 238),
            "#7B68EE",
            "(r: 123, g: 104, b: 238)",
        ),
        (
            "DarkGray",
            (169, 169, 169),
            "#A9A9A9",
            "(r: 169, g: 169, b: 169)",
        ),
        (
            "RosyBrown",
            (188, 143, 143),
            "#BC8F8F",
            "(r: 188, g: 143, b: 143)",
        ),
        (
            "SlateGray",
            (112, 128, 144),
            "#708090",
            "(r: 112, g: 128, b: 144)",
        ),
        (
            "DarkTurquoise",
            (0, 206, 209),
            "#00CED1",
            "(r: 0, g: 206, b: 209)",
        ),
        (
            "RosyBrown",
            (188, 143, 143),
            "#BC8F8F",
            "(r: 188, g: 143, b: 143)",
        ),
        (
            "GhostWhite",
            (248, 248, 255),
            "#F8F8FF",
            "(r: 248, g: 248, b: 255)",
        ),
        (
            "Turquoise",
            (64, 224, 208),
            "#40E0D0",
            "(r: 64, g: 224, b: 208)",
        ),
        (
            "Fuchsia",
            (255, 0, 255),
            "#FF00FF",
            "(r: 255, g: 0, b: 255)",
        ),
        (
            "SlateBlue",
            (106, 90, 205),
            "#6A5ACD",
            "(r: 106, g: 90, b: 205)",
        ),
        (
            "LightSlateGray",
            (119, 136, 153),
            "#778899",
            "(r: 119, g: 136, b: 153)",
        ),
        (
            "DarkGoldenrod",
            (184, 134, 11),
            "#B8860B",
            "(r: 184, g: 134, b: 11)",
        ),
        ("Peru", (205, 133, 63), "#CD853F", "(r: 205, g: 133, b: 63)"),
        (
            "OliveDrab",
            (107, 142, 35),
            "#6B8E23",
            "(r: 107, g: 142, b: 35)",
        ),
        (
            "OldLace",
            (253, 245, 230),
            "#FDF5E6",
            "(r: 253, g: 245, b: 230)",
        ),
    ];

    #[test]
    fn traduce_rgb2hex() {
        for case in VALID_TEST_CASES {
            let (_, (red, green, blue), hex_str, _) = case;
            assert_eq!(TrueColor::rgb2hex(red, green, blue), hex_str)
        }
    }

    #[test]
    fn traduce_hex2rgb() {
        for case in VALID_TEST_CASES {
            let (_, _, hex_str, rgb_str) = case;
            assert_eq!(TrueColor::hex2rgb(hex_str.to_string()), rgb_str)
        }
    }
}
