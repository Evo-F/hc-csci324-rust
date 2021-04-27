use rand::seq::IteratorRandom;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

fn validate_word(word: String, diff: u32) -> bool {
    //Can I just say, I love the match statement over the switch statement in other languages.
    //If difficulty is easy or medium (1 or 2), max length gets set.
    //Otherwise, there isn't one.
    let max_length = match diff {
        1 => 5,
        2 => 9,
        _ => 0
    }; //for difficulties 3 and 4, there is no maximum length.

    let min_length = match diff {
        2 => 6,
        3 => 10,
        _ => 0
    }; //for difficulties 1 and 4, there is no minimum length.

    let string_length = word.len() as u32;

    return match diff {
        1 if string_length <= max_length => true,
        2 if string_length <= max_length && string_length >= min_length => true,
        3 if string_length >= min_length => true,
        4 => true,
        _ => false
    }
}


// Randomly selects a word from the filepath and returns it as a String.
// The number provided as diff corresponds to a difficulty option. See below.
pub fn choose_word(diff: u32) -> String {

    let filename = "dictionary.txt";
    if filename.contains("testdict") {
        println!("DEV NOTICE: You are using the test dictionary!")
    }

    //This function supports difficulty options! Depending on what is specified by diff,
    //this function will only return words of a certain (arbitrary) minimum and maximum length.
    /*
    Difficulty 1 - Words of length 6 or less.
    Difficulty 2 - Words of length 6 to 9.
    Difficulty 3 - Words of length 9 or greater.
    Difficulty 4 - All words allowed.
     */

    //We love OpenOptions!
    //Attempts to open the specified FilePath in read-only mode.
    //The .expect() is for error handling.
    let file = OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("ERROR: File not found!");

    let reader = BufReader::new(file); //Standard buffered reader

    //reader.lines() returns an iterator over the lines in the file - this iterator is a Lines object.
    let lines = reader.lines();
    let filtered_lines = lines
        .filter(|s| validate_word(s
                                      .as_ref()
                                      .expect("ERROR: Could not read string!")
                                      .to_string(), diff));

    let word_to_return = filtered_lines
        .choose(&mut rand::thread_rng())
        .unwrap()
        .expect("ERROR: Could not read lines of the file!");

    return word_to_return;
}

// Appends the word provided as a parameter to the dictionary file
pub fn add_word(word: &str) {

    let filename = "src/dictionary.txt";

    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("ERROR: File not found!");
    writeln!(file, "{}", word).expect("ERROR: Could not write to file!");
}