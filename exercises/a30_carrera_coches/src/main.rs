/*
Reto #46: La carrera de coches
Crea un programa que simule la competición de dos coches en una pista.
- Los dos coches estarán representados por 🚙 y 🚗. Y la meta por 🏁.
- Cada pista tendrá entre 1 y 3 árboles 🌲 colocados de forma aleatoria.
- Las dos pistas tendrán una longitud configurable de guiones bajos "_".
- Los coches comenzarán en la parte derecha de las pistas. Ejemplo:
  🏁____🌲_____🚙
  🏁_🌲____🌲___🚗

El juego se desarrolla por turnos de forma automática, y cada segundo
se realiza una acción sobre los coches (moviéndose a la vez), hasta que
uno de ellos (o los dos a la vez) llega a la meta.
- Acciones:
  - Avanzar entre 1 a 3 posiciones hacia la meta.
  - Si al avanzar, el coche finaliza en la posición de un árbol,
    se muestra 💥 y no avanza durante un turno.
  - Cada turno se imprimen las pistas y sus elementos.
  - Cuando la carrera finalice, se muestra el coche ganador o el empate.
 */

fn main() {
    println!("Hello, world!");
}

/// Modulo encargado de la lógica interna del juego
pub mod game {

    /// Número de ticks el carro no puede moverse después de estrellarse.
    const CRASHED_TICKS: usize = 1;
    /// Tamaño mínimo permitido para el circuito
    const TRACK_MIN_LENGTH: usize = 10;
    /// Tamaño máximo permitido para el circuito
    const TRACK_MAX_LENGTH: usize = 30;

    // Biblioteca para la generación de valores aleatorios
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// Representa el estado del carro. El valor numérico, cuando esta presente,
    /// indica el tiempo que lleva en ese estado
    #[derive(Default)]
    #[non_exhaustive]
    pub enum CarStatus {
        #[default]
        OK,
        Crashed(usize),
    }

    #[non_exhaustive]
    #[derive(PartialEq)]
    pub enum CarEvent {
        /// Efectivamente se mueve
        Advances,
        /// Se choca
        Crash,
        /// A cruzado la linea de meta
        FinishLine,
    }

    #[derive(Default)]
    pub struct Car {
        position: usize,
        status: CarStatus,
    }

    impl Car {
        /// Indica el número de posiciones de movimiento hacia la meta
        fn try_to_advance(&self) -> usize {
            match self.status {
                CarStatus::OK => StdRng::from_entropy().gen_range(1..=3),
                CarStatus::Crashed(_) => 0,
            }
        }

        fn check_status(&mut self) {
            if let CarStatus::Crashed(t) = self.status {
                if t >= CRASHED_TICKS {
                    self.status = CarStatus::OK
                }
                self.status = CarStatus::Crashed(t - 1)
            }
        }
    }

    /// Representación de un arbol
    pub struct Tree {
        position: usize,
    }

    pub struct Track {
        length: usize,
        car: Car,
        trees: Vec<Tree>,
    }

    impl Track {
        fn new(car: Car, length: usize) -> Self {
            // escoge aleatoriamente el número de árboles y sus posiciones
            // Los arboles NO pueden estar ni la meta ni el punto de salida.
            let tree_number: u8 = StdRng::from_entropy().gen_range(1..=3);
            let mut trees: Vec<Tree> = Vec::with_capacity(tree_number as usize);
            for _id in 0..tree_number {
                let position: usize = StdRng::from_entropy().gen_range(1..tree_number as usize);
                trees.push(Tree { position })
            }

            Self { length, trees, car }
        }
        /// Corre la lógica correspondiente para cada paso de tiempo
        fn tick(&mut self) -> CarEvent {
            // el carro valida sí puede moverse en este turno
            self.car.check_status();
            let next_car_position = self.car.try_to_advance() + self.car.position;
            // Verifica el carro no haya pasado la meta
            if next_car_position >= self.length {
                return CarEvent::FinishLine;
            }
            // Como no ha cruzado la meta, actualiza su estado y evalúa las consecuencias
            self.car.position = next_car_position;

            if self.car_collision() {
                self.car.status = CarStatus::Crashed(CRASHED_TICKS);
                return CarEvent::Crash;
            }
            // No ha pasado la linea de meta ni chocado
            CarEvent::Advances
        }

        fn car_collision(&self) -> bool {
            self.trees
                .iter()
                .any(|tree| tree.position == self.car.position)
        }

        pub fn get_tree_positions(&self) -> Vec<&usize> {
            self.trees.iter().map(|tree| &tree.position).collect()
        }

        pub fn get_length(&self) -> &usize {
            &self.length
        }

        pub fn get_car_info(&self) -> (&usize, &CarStatus) {
            (&self.car.position, &self.car.status)
        }
    }

    pub enum Winner {
        A,
        B,
        Both,
    }

    pub struct Race {
        track_a: Track,
        track_b: Track,
        winner: Option<Winner>,
    }

    impl Default for Race {
        fn default() -> Self {
            //randomly chooses track length
            let length: usize =
                StdRng::from_entropy().gen_range(TRACK_MIN_LENGTH..=TRACK_MAX_LENGTH);
            Self {
                track_a: Track::new(Car::default(), length),
                track_b: Track::new(Car::default(), length),
                winner: None,
            }
        }
    }

    impl Race {
        /// realiza la lógica de cada paso de tiempo. Sí hay un ganador, retorna None,
        /// de lo contrario, retorna que eventos ocurrieron
        pub fn tick(&mut self) -> Option<(CarEvent, CarEvent)> {
            // No realiza nada sí ya hay ganador
            if self.winner.is_some() {
                return None;
            }
            // Ejecuta el paso del tiempo
            let event_track_a = self.track_a.tick();
            let event_track_b = self.track_b.tick();

            // Evalúa consecuencias
            let a_wins = event_track_a == CarEvent::FinishLine;
            let b_wins = event_track_b == CarEvent::FinishLine;

            if !a_wins && !b_wins {
                return Some((event_track_a, event_track_b));
            }

            if a_wins && b_wins {
                self.winner = Some(Winner::Both);
            }

            if a_wins {
                self.winner = Some(Winner::A);
            } else {
                self.winner = Some(Winner::B);
            }

            None
        }

        pub fn get_track_a(&self) -> &Track {
            &self.track_a
        }
        pub fn get_track_b(&self) -> &Track {
            &self.track_b
        }
        pub fn winner(&self) -> &Option<Winner> {
            &self.winner
        }
    }
}
