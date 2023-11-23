// author: Hans D. Escobar H. (hdescobarh)

/* Reto #45: El calendario de aDEViento 2023
Crea un programa que simule el mecanismo de participación de aDEViento:
- Mediante la terminal, el programa te preguntará si quieres añadir y borrar
participantes, mostrarlos, lanzar el sorteo o salir.
- Si seleccionas añadir un participante, podrás escribir su nombre y pulsar enter.
- Si seleccionas añadir un participante, y este ya existe, avisarás de ello. (Y no lo duplicarás)
- Si seleccionas mostrar los participantes, se listarán todos.
- Si seleccionas eliminar un participante, podrás escribir su nombre y
pulsar enter. (Avisando de si lo has eliminado o el nombre no existe)
- Si seleccionas realizar el sorteo, elegirás una persona al azar y se eliminará del listado.
- Si seleccionas salir, el programa finalizará.
 */

fn main() {}

pub mod cli {}

pub mod giveaway {
    use crate::giveaway_data::{ErrorKind, Participant, Repository};
    use rand::seq::SliceRandom;
    use std::fmt::Display;

    const MSG_ERR_NOT_FOUND: &str = "The participant does not exists.";
    const MSG_ERR_ALREADY_EXIST: &str = "The participant already exists.";
    const MSG_ERR_EMPTY_DATA: &str = "There are not participants.";
    const MSG_SUCCESS_ADDING: &str = "The participant was added.";
    const MSG_SUCCESS_DELETING: &str = "The Participant was removed.";

    pub struct Giveaway {
        repository: Box<dyn Repository + 'static>,
    }

    impl Giveaway {
        pub fn new(repository: impl Repository + 'static) -> Self {
            Self {
                repository: Box::new(repository),
            }
        }

        pub fn request(&mut self, kind: Request, username: Option<&str>) -> String {
            match kind {
                Request::AddParticipant => self.add_participant(username.unwrap()),
                Request::ListParticipants => self.get_all_participants(),
                Request::RemoveParticipant => self.remove_participant(username.unwrap()),
                Request::Draw => self.draw(),
            }
        }

        /// Agrega un nuevo participante
        fn add_participant(&mut self, username: &str) -> String {
            match self.repository.create(Participant {
                username: username.to_owned(),
            }) {
                Ok(_) => MSG_SUCCESS_ADDING.to_string(),
                Err(e) => format!("{e}"),
            }
        }

        /// Lista todos los participantes
        fn get_all_participants(&self) -> String {
            match self.repository.read_all() {
                Ok(l) => String::from_iter(
                    l.into_iter()
                        .map(|participant| format!("- {}\n", participant.username)),
                ),
                Err(e) => format!("{e}"),
            }
        }

        /// Remueve por nombre de usuario a un participante
        fn remove_participant(&mut self, username: &str) -> String {
            match self.repository.delete(username) {
                Ok(_) => MSG_SUCCESS_DELETING.to_string(),
                Err(e) => format!("{e}"),
            }
        }

        /// Obtiene un ganador y lo remueve de la lista de participantes
        fn draw(&mut self) -> String {
            let mut rng = rand::thread_rng();
            let participants = match self.repository.read_all() {
                Ok(l) => l,
                Err(e) => return format!("{e}"),
            };

            let winner = match participants.choose(&mut rng) {
                Some(participant) => participant.username.to_owned(),
                None => return format!("{}", ErrorKind::Empty),
            };
            match self.repository.delete(&winner) {
                Ok(_) => winner,
                Err(e) => format!("{e}"),
            }
        }
    }

    pub enum Request {
        AddParticipant,
        ListParticipants,
        RemoveParticipant,
        Draw,
    }

    impl Display for ErrorKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg = match self {
                ErrorKind::NotFound => MSG_ERR_NOT_FOUND,
                ErrorKind::AlreadyExist => MSG_ERR_ALREADY_EXIST,
                ErrorKind::Empty => MSG_ERR_EMPTY_DATA,
            };
            write!(f, "{msg}")
        }
    }
}

pub mod giveaway_data {
    // Las nombres de usuario de Twitter son únicos, por lo que pueden usarse como identificadores únicos del participante
    #[derive(PartialEq, Clone)]
    pub struct Participant {
        pub username: String,
    }

    pub trait Repository {
        fn create(&mut self, participant: Participant) -> Result<(), ErrorKind>;
        fn read_all(&self) -> Result<Vec<Participant>, ErrorKind>;
        fn delete(&mut self, username: &str) -> Result<(), ErrorKind>;
    }

    #[non_exhaustive]
    pub enum ErrorKind {
        NotFound,
        AlreadyExist,
        Empty,
    }

    // Para el ejercicio la "conexión" será falseada por una simple colección
    pub struct MockRepository {
        connection: Vec<Participant>,
    }

    impl MockRepository {
        fn get_username_index(&self, username: &str) -> Result<usize, ErrorKind> {
            for (index, entry) in self.connection.iter().enumerate() {
                if entry.username == username {
                    return Ok(index);
                }
            }
            Err(ErrorKind::NotFound)
        }
    }

    impl Default for MockRepository {
        fn default() -> Self {
            Self {
                connection: Vec::from(["AkiraToriyama", "mouredev", "midudev", "sama"].map(|i| {
                    Participant {
                        username: i.to_string(),
                    }
                })),
            }
        }
    }

    impl Repository for MockRepository {
        fn create(&mut self, participant: Participant) -> Result<(), ErrorKind> {
            if self.connection.contains(&participant) {
                return Err(ErrorKind::AlreadyExist);
            }

            self.connection.push(participant);
            Ok(())
        }

        fn read_all(&self) -> Result<Vec<Participant>, ErrorKind> {
            if self.connection.is_empty() {
                Err(ErrorKind::Empty)
            } else {
                Ok(Vec::from_iter(self.connection.iter().cloned()))
            }
        }

        fn delete(&mut self, username: &str) -> Result<(), ErrorKind> {
            let index = self.get_username_index(username)?;
            self.connection.swap_remove(index);
            Ok(())
        }
    }
}
