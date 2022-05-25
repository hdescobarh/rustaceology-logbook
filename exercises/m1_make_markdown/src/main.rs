use glob::glob; // https://docs.rs/glob/latest/glob/
use std::fs::File; // https://doc.rust-lang.org/stable/std/fs/struct.File.html
use std::io::prelude::*;
use std::path::PathBuf;

fn get_paths(notes_location: String) -> Vec<PathBuf> {
    let notes_rs_pattern = format!("{}{}", notes_location, "/**/*.rs");

    let path_list: Vec<PathBuf> = glob(&notes_rs_pattern)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    path_list
}

fn make_md_block(file_path: &PathBuf) -> String {
    let file_name: String = file_path
        .display()
        .to_string()
        .clone()
        .split("\\")
        .collect::<Vec<&str>>()[3]
        .to_owned();

    let mut text: String = String::from(format!("{}{}", "\n## ", file_name));

    text.push_str("\n\n```rust\n");

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
        Ok(file) => file,
    };

    match file.read_to_string(&mut text) {
        Err(why) => panic!("couldn't read {}: {}", file_path.display(), why),
        Ok(_) => {
            text.push_str("```\n");
            return text;
            //print!("{} contains:\n{}", file_path.display(), text)
        }
    }
}

fn main() -> std::io::Result<()> {
    let notes_location: String = "./../../notes".to_owned();
    let notes_paths = get_paths(notes_location);

    let mut notes_all = String::from("# My notes extract\n\n");

    for note_path in notes_paths {
        notes_all.push_str(make_md_block(&note_path).as_str())
    }

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    let mut file = File::create("mynotes.md")?;
    file.write_all(notes_all.as_bytes())?;
    Ok(())
}
