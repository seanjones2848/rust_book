use std::io;

fn main() {
    println!("Convert your words to Pig Latin!");

    println!("Please input your phrase.");

    let mut phrase = String::new();

    io::stdin()
        .read_line(&mut phrase)
        .expect("Failed to read line");


    println!("Your phrase is: {phrase}");

    let mut pig_latin_phrase = String::new();

    for (idx, word) in phrase.split_whitespace().enumerate() {
        let mut chars = word.chars();
        let first_letter = chars.next();
        match first_letter {
            Some(letter) => {
                let rest_of_word = chars.as_str();

                let pig_latin_word = if word.chars().count() > 1 {
                    format!("{}-{}ay", rest_of_word, letter.to_lowercase())
                } else {
                    format!("{}ay", letter.to_lowercase())
                };
                
                if idx > 0 {
                    pig_latin_phrase += " "
                }
                pig_latin_phrase += &pig_latin_word;
            },
            None => (),
        }
    }
    println!("Your phrase in Pig Latin is: {pig_latin_phrase}");
}
