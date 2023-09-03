const CARACTER_ACTIVOS: char = '🔳';
const CARACTER_INACTIVOS: char = '🔲';
const FILAS_TABLERO: usize = 10;
const COLUMNAS_TABLERO: usize = 10;

fn main() {
    let mut pantalla = Pantalla::default();
    pantalla.imprimir_estado();
    pantalla.mover_tetromino();
    pantalla.imprimir_estado();
    pantalla.mover_tetromino();
    pantalla.imprimir_estado();
}

struct Coordenada {
    fila: isize,
    columna: isize,
}

struct Tetromino {
    // Cada Tetromino es un cuadrado de 3x3 o 4x4
    // Esta es la coordenada en el tablero del cuadrado de la esquina inferior izquierda
    coordenada_referencia: Coordenada,
    // El origen de las coordenadas internas (0,0) es el cuadrado central
    activos: Vec<Coordenada>,
}

impl Tetromino {
    fn new(coordenada_real: Coordenada) -> Self {
        // para esta prueba, la figura es el Tetromino J (L invertida)
        let figura_activos = [[1, -1], [0, -1], [0, 0], [0, 1]];
        let activos = figura_activos
            .into_iter()
            .map(|[fila, columna]| Coordenada { fila, columna })
            .collect();
        Tetromino {
            coordenada_referencia: coordenada_real,
            activos,
        }
    }

    fn traducir_a_coordenadas_reales(&self) -> Vec<Coordenada> {
        self.activos
            .iter()
            .map(|coordenada| Coordenada {
                fila: self.coordenada_referencia.fila - (1 + coordenada.fila),
                columna: self.coordenada_referencia.columna + (1 + coordenada.columna),
            })
            .collect()
    }

    fn realizar_movimiento(&self) -> Tetromino {
        // PRIMERO realiza la transformación y almacena temporalmente
        // EJEMPLO CON ROTAR
        let nuevos_activos = self.rotar();
        let nueva_coordenada_referencia = Coordenada {
            fila: self.coordenada_referencia.fila,
            columna: self.coordenada_referencia.columna,
        };
        // SEGUNDO el tablero mira y valida el movimiento como legal
        Tetromino {
            coordenada_referencia: nueva_coordenada_referencia,
            activos: nuevos_activos,
        }
    }

    fn rotar(&self) -> Vec<Coordenada> {
        self.activos
            .iter()
            .map(|coor| Coordenada {
                fila: coor.columna,
                columna: -coor.fila,
            })
            .collect()
    }
}

struct Pantalla {
    estado: [[char; COLUMNAS_TABLERO]; FILAS_TABLERO],
    tetromino_activo: Tetromino, // Empieza con un activo para la prueba de concepto
}

impl Pantalla {
    fn default() -> Self {
        let estado = [[CARACTER_INACTIVOS; COLUMNAS_TABLERO]; FILAS_TABLERO];
        let tetromino_activo = Tetromino::new(Coordenada {
            fila: 3,
            columna: 0,
        });

        Pantalla {
            estado,
            tetromino_activo,
        }
    }

    fn imprimir_estado(&self) {
        let mut para_imprimir = self.estado;

        for coord in self.tetromino_activo.traducir_a_coordenadas_reales() {
            para_imprimir[coord.fila as usize][coord.columna as usize] = CARACTER_ACTIVOS;
        }

        for fila in para_imprimir {
            println!("\t{}", String::from_iter(fila.iter()));
        }

        println!("\n");
    }

    /**
     * Verifica si las nuevas coordenadas del tetrominó esta en los limites
     */
    fn validar_bordes(tetromino: &Tetromino) -> bool {
        for coor in tetromino.traducir_a_coordenadas_reales() {
            let fila_ilegal = coor.fila < 0 || coor.fila as usize >= FILAS_TABLERO;
            let columna_ilegal = coor.columna < 0 || coor.columna as usize >= COLUMNAS_TABLERO;
            if fila_ilegal || columna_ilegal {
                return false;
            }
        }
        true
    }

    fn mover_tetromino(&mut self) {
        // recibe orden de movimiento
        let tetromino = self.tetromino_activo.realizar_movimiento();
        // envia a la ficha el tipo de movimient que debe hacer y espera coordenadas de activos
        // valida esas coordenadas: esta dentro del tablero?, esta libre el espacio?
        let bordes_validados = Pantalla::validar_bordes(&tetromino);
        // sí es valido envia OK a la ficha e imprime con nuevo estado
        self.tetromino_activo = tetromino;
        // si no es valida, deniega a la ficha e imprime estado sin modificar
    }
}

enum Movimiento {
    Rotacion,
    Derecha,
    Izquierda,
    Abajo,
}
