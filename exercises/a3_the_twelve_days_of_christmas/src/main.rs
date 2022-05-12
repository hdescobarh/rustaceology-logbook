fn song_printer() {
    let song_phrases: [&str; 13] = [
        "On the twelfth day of Christmas, my true love sent to me",
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    let max_length = song_phrases.len();
    let mut remanining_verses = max_length;

    while remanining_verses > 1 {
        println!("[Verse {}]", max_length - remanining_verses + 1);
        println!("{}", song_phrases[0]);
        for phrase in &song_phrases[remanining_verses - 1..max_length] {
            println!("{}", phrase);
        }
        println!("");
        remanining_verses = remanining_verses - 1;
    }
}

fn main() {
    song_printer();
}
