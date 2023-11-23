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

pub mod giveaway {
    use crate::giveaway_data::*;
    use rand::seq::SliceRandom;

    pub struct Giveaway {
        repository: Box<dyn Repository + 'static>,
    }

    impl Giveaway {
        pub fn new(repository: impl Repository + 'static) -> Self {
            Self {
                repository: Box::new(repository),
            }
        }

        pub fn request(kind: Request) -> String {
            match kind {
                Request::AddParticipant => todo!(),
                Request::ListParticipants => todo!(),
                Request::RemoveParticipant => todo!(),
                Request::Draw => todo!(),
            }
        }

        /// Agrega un nuevo participante
        fn add_participant(&mut self, username: &str) -> Result<(), ErrorKind> {
            self.repository.create(Participant {
                username: username.to_owned(),
            })
        }

        /// Lista todos los participantes
        fn all_participants(self) -> Result<Vec<String>, ErrorKind> {
            Ok(self
                .repository
                .read_all()?
                .into_iter()
                .map(|participant| participant.username)
                .collect::<Vec<String>>())
        }

        /// Remueve por nombre de usuario a un participante
        fn remove_participant(&mut self, username: &str) -> Result<(), ErrorKind> {
            self.repository.delete(username)
        }

        /// Obtiene un ganador y lo remueve de la lista de participantes
        fn draw(&mut self) -> Result<String, ErrorKind> {
            let mut rng = rand::thread_rng();
            let winner = {
                match self.repository.read_all()?.choose(&mut rng) {
                    Some(participant) => participant.username.to_owned(),
                    None => return Err(ErrorKind::Empty),
                }
            };
            match self.repository.delete(&winner) {
                Ok(_) => Ok(winner),
                Err(error) => Err(error),
            }
        }
    }

    pub enum Request {
        AddParticipant,
        ListParticipants,
        RemoveParticipant,
        Draw,
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
