fn main() {
    let test_list = [
        "Hello world!",
        "¡Hola mundo!",
        "Γειά σου Κόσμε!",
        "こんにちは世界！",
        "What is Lorem Ipsum?
Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
    ]
    .map(|x| String::from(x));

    for test in test_list {
        println!("testing...");
        println!(
            "Text:\n{}\nHas {} words (simple) \nHas {} words (scratch).",
            test,
            word_count_simple(&test),
            word_count_from_scratch(&test)
        );
    }
}

fn word_count_simple(string: &String) -> u128 {
    let mut counter = 0u128;
    // string.split_whitespace.count() is simpler, but does not deal whith the usize overflow
    for _ in string.split_whitespace() {
        counter = counter + 1;
    }
    counter
}

fn word_count_from_scratch(string: &String) -> u128 {
    let unicode_whitespaces = [
        0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x0020, 0x0085, 0x00A0, 0x1680, 0x2000, 0x2001,
        0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200A, 0x2028, 0x2029,
        0x202F, 0x205F, 0x3000,
    ]
    .map(|x| char::from_u32(x).unwrap());

    let mut whitespace_counter = 1u128;

    for character in string.chars() {
        for whitespace in unicode_whitespaces {
            if character == whitespace {
                whitespace_counter = whitespace_counter + 1;
                break;
            }
        }
    }
    whitespace_counter
}
