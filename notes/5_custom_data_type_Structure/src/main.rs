fn main() {
    // Introducing structs
    basics_struct();

    // Tuple structs: as structs but without names for the fields
    struct Coordinates(i64, i64, u64);
    let _vector = Coordinates(14, 15, 9);

    // Unit-Like structs: as tuple structs, but whithout fields
    struct NullElement;
    let _null_element = NullElement;
}

fn basics_struct() {
    // Structure instantiation syntax
    let population1 = Population {
        genre: String::from("Homo"),
        specie: String::from("sapiens"),
        size: 7000,
    };

    // Structure instantiation shorthand
    let population2 =
        structure_instantiation_shorthand(String::from("Homo"), String::from("habilis"), 0u32);

    // update with move
    let _population3 = Population {
        specie: String::from("neanderthalensis"),
        ..population2 // COPY or MOVE the remaining fields. String is on the heap, so it is moved
    };

    println!("population2.genre was moved; then, population2 is no longer accesible");

    // update with copy
    let population4 = Population {
        genre: String::from("Pan"),
        specie: String::from("fictitious"),
        ..population1 // COPY or MOVE the remaining fields. u32 is only-stack data, so it is copied
    };

    println!(
        "Population 1:\n{:?}\nPopulation 4:\n{:?}\n",
        population1, population4
    );

    println!("population1 is extinct: {}", population1.it_is_extinct())
}

// Structure definition
#[derive(Debug)]
#[allow(dead_code)]
struct Population {
    genre: String,
    specie: String,
    size: u32,
}

// Associated functions
impl Population {
    // METHODS are associated functions which requires an instance of the struct (i.e. uses &self)
    fn it_is_extinct(&self) -> bool {
        // '&self' is a shorthand for 'self: &self'
        if self.size > 0 {
            false
        } else {
            true
        }
    }
    // associated functions which are not methods usually are used as constructors
    #[allow(dead_code)]
    fn homosapiens(size: u32) -> Population {
        Population {
            genre: String::from("Homo"), //String::from() itself is an example of constructor. Notice the "::"
            specie: String::from("sapiens"),
            size,
        }
    }
}

fn structure_instantiation_shorthand(genre: String, specie: String, size: u32) -> Population {
    Population {
        genre: genre, // Use complete sintax
        specie,       // or use field init short hand
        size,
    }
}
