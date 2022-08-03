fn main() {
    let test_strings: [String; 6] = [
        "¡Caballero!",
        "¡caballero!",
        "Caballero!",
        "騎馬武者",
        "ﾊﾝｽ",
        "ハンス",
    ]
    .map(|x| String::from(x));

    let testing_pairs: [[usize; 2]; 5] = [[0, 1], [0, 2], [0, 3], [2, 3], [4, 5]];

    println!("Remember Rust strings are UTF-8–encoded\n");
    for pair in testing_pairs {
        println!("\nTesting...");
        let string1: &String = &test_strings[pair[0]];
        let string2: &String = &test_strings[pair[1]];
        println!(
            "String 1:\n{}\nString 2:\n{}\nBitwise Hamming distance:\n{:?}",
            string1,
            string2,
            bitwise_hamming_distance(string1, string2)
        )
    }
}

enum SameLenght {
    True(Vec<u8>, Vec<u8>, usize),
    False,
}

impl SameLenght {
    fn check_same_byte_lenght(string1: &String, string2: &String) -> SameLenght {
        //return an enum
        let bytes1 = string1.as_bytes();
        let bytes2 = string2.as_bytes();
        let lenght = bytes1.len();

        if lenght == bytes2.len() {
            return SameLenght::True(bytes1.to_owned(), bytes2.to_owned(), lenght);
        } else {
            return SameLenght::False;
        }
    }
}

fn bitwise_hamming_distance(string1: &String, string2: &String) -> Result<u32, &'static str> {
    match SameLenght::check_same_byte_lenght(string1, string2) {
        SameLenght::True(bytes1, bytes2, lenght) => {
            let mut counter = 0u32;
            for index in 0..lenght {
                counter = counter + (bytes1[index] ^ bytes2[index]).count_ones();
            }
            Result::Ok(counter)
        }
        SameLenght::False => Result::Err("The strings do not have the same lenght"),
    }
}
