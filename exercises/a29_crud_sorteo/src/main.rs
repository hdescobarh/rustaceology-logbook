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

    use rand::distributions::Slice;
    use rand::Rng;

    use crate::giveaway_data::*;

    pub struct Giveaway {
        participants: Box<dyn for<'a> Repository<'a, String> + 'static>,
    }

    impl Giveaway {
        /// Agrega un nuevo participante
        pub fn add_participant(&mut self, username: String) -> Result<(), ErrorKind> {
            self.participants.create(username)
        }

        /// Lista todos los participantes
        pub fn all_participants(&self) -> Result<&[String], ErrorKind> {
            self.participants.read_all()
        }

        /// Remueve por nombre de usuario a un participante
        pub fn remove_participant(&mut self, username: &String) -> Result<(), ErrorKind> {
            self.participants.delete(username)
        }

        /// Obtiene un ganador y lo remueve de la lista de participantes
        pub fn draw(&mut self) -> Result<String, ErrorKind> {
            let draw_distribution = match Slice::new(self.participants.read_all()?) {
                Ok(d) => d,
                Err(_) => return Err(ErrorKind::Empty),
            };

            let winner = rand::thread_rng().sample(draw_distribution).to_owned();
            match self.participants.delete(&winner) {
                Ok(_) => Ok(winner),
                Err(error) => Err(error),
            }
        }
    }
}

pub mod giveaway_data {
    // Las nombres de usuario de Twitter son únicos, por lo que pueden usarse como identificadores únicos del participante
    pub trait Repository<'a, K> {
        fn create(&mut self, username: K) -> Result<(), ErrorKind>;
        fn read_all(&'a self) -> Result<&'a [K], ErrorKind>;
        fn delete(&mut self, username: &K) -> Result<(), ErrorKind>;
    }

    #[non_exhaustive]
    pub enum ErrorKind {
        NotFound,
        AlreadyExist,
        Empty,
    }

    // Para el ejercicio, la "conexión" sera una simple lista, pero puede ser cualquier recurso
    pub struct ListParticipants<'a> {
        database_connection: &'a mut Vec<String>,
    }

    impl<'a> ListParticipants<'a> {
        pub fn new(database_connection: &'a mut Vec<String>) -> Self {
            Self {
                database_connection,
            }
        }

        fn get_username_index(&self, username: &str) -> Result<usize, ErrorKind> {
            for (index, entry) in self.database_connection.iter().enumerate() {
                if entry == username {
                    return Ok(index);
                }
            }
            Err(ErrorKind::NotFound)
        }
    }

    impl<'a> Repository<'a, String> for ListParticipants<'a> {
        fn create(&mut self, username: String) -> Result<(), ErrorKind> {
            if self.database_connection.contains(&username) {
                return Err(ErrorKind::AlreadyExist);
            }

            self.database_connection.push(username);
            Ok(())
        }

        fn read_all(&'a self) -> Result<&'a [String], ErrorKind> {
            if self.database_connection.is_empty() {
                Err(ErrorKind::Empty)
            } else {
                Ok(self.database_connection)
            }
        }

        fn delete(&mut self, username: &String) -> Result<(), ErrorKind> {
            let index = self.get_username_index(username)?;
            self.database_connection.swap_remove(index);
            Ok(())
        }
    }
}
