// author: hdescobarh (Hans D. Escobar H.)

/*
Reto #34: El TXT https://github.com/mouredev/retos-programacion-2023

Crea un programa capaz de interactuar con un fichero TXT.
IMPORTANTE: El fichero TXT NO debe subirse como parte de la corrección.
Únicamente el código.
    - Si no existe, debe crear un fichero llamado "text.txt".
    - Desde el programa debes ser capaz de introducir texto por consola y guardarlo
        en una nueva línea cada vez que se pulse el botón "Enter".
    - Si el fichero existe, el programa tiene que dar la opción de seguir escribiendo
        a continuación o borrar su contenido y comenzar desde el principio.
    - Si se selecciona continuar escribiendo, se tiene que mostrar por consola
        el texto que ya posee el fichero.
 */

use mi_editor::Editor;
use std::path::PathBuf;

const RUTA_FICHERO_STR: &str = "./text.txt";

fn main() {
    let mut input_buffer = String::new();
    // Valida que la ruta sea accesible
    let ruta: PathBuf = PathBuf::from(RUTA_FICHERO_STR);
    let mut editor_activo: Editor;
    if ruta.try_exists().unwrap() {
        // preguntar sí sobrescribir
        let sobrescribir = cli::obtener_opcion_sobrescritura(&mut input_buffer);
        editor_activo = Editor::desde_fichero_existente(ruta, sobrescribir);
        if !sobrescribir {
            println!("{}", editor_activo.mostrar_buffer());
        }
    } else {
        editor_activo = Editor::desde_fichero_nuevo(ruta)
    };

    loop {
        cli::input_de_linea_nueva(&mut input_buffer);
        editor_activo.escribir_linea(&input_buffer).unwrap();
    }
}

pub mod mi_editor {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;

    pub struct Editor {
        _ruta: PathBuf,
        fichero: File,
        buffer: String,
    }

    impl Editor {
        pub fn desde_fichero_nuevo(ruta: PathBuf) -> Self {
            // crea directorios sí no existen, luego crea el archivo
            if let Some(parent_path) = ruta.parent() {
                std::fs::create_dir_all(parent_path).unwrap()
            };
            let fichero = OpenOptions::new()
                .read(true)
                .create_new(true) // create_new asegura que el archivo no exista
                .open(&ruta)
                .unwrap();

            Self {
                _ruta: ruta,
                fichero,
                buffer: String::new(),
            }
        }

        pub fn desde_fichero_existente(ruta: PathBuf, sobrescribir: bool) -> Self {
            let mut fichero = OpenOptions::new()
                .read(true)
                .write(true) // truncate requiere write(true)
                .append(!sobrescribir) // sí write y append son true, aplica append
                .truncate(sobrescribir)
                .open(&ruta)
                .unwrap();

            let mut buffer = String::new();
            fichero.read_to_string(&mut buffer).unwrap();
            Self {
                _ruta: ruta,
                fichero,
                buffer,
            }
        }

        pub fn escribir_linea(&mut self, linea: &String) -> std::io::Result<()> {
            //self.fichero.write_all(&buffer)
            write!(self.fichero, "{}", linea)
        }

        pub fn mostrar_buffer(&self) -> &String {
            &self.buffer
        }

        pub fn _mostrar_ruta(&self) -> &PathBuf {
            &self._ruta
        }
    }
}
pub mod cli {
    const PROMPT: &str = "Ingrese el texto >";
    const MSJ_SOBRESCRIBIR: &str = "
    El fichero ya existe, desea sobrescribirlo?
    0: No
    1: Sí:";
    const MSJ_OPCION_NO_VALIDA: &str = "El número ingresado no es una opción valida.";

    use std::io;

    // limpia el buffer y espera el input del usuario
    pub fn input_de_linea_nueva(input_buffer: &mut String) {
        input_buffer.clear();
        println!("{}", PROMPT);
        io::stdin().read_line(input_buffer).unwrap();
    }

    // pregunta por sí deseea sobreescribir un archivo existente y retorna una opcion valida
    pub fn obtener_opcion_sobrescritura(input_buffer: &mut String) -> bool {
        loop {
            println!("{}", MSJ_SOBRESCRIBIR);
            io::stdin().read_line(input_buffer).unwrap();

            match input_buffer.trim().parse::<u8>() {
                Ok(opcion) if opcion == 0 => return false,
                Ok(opcion) if opcion == 1 => return true,
                _ => println!("{}", MSJ_OPCION_NO_VALIDA),
            }
        }
    }
}
