//TODO: This file contains code that was borrowed from this link:
//               https://stackoverflow.com/questions/50788009/how-do-i-get-a-random-line-from-a-file
// This code is present solely for testing. It needs to be modified and properly cited.

use rand::seq::IteratorRandom;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

// Randomly selects a word from the filepath and returns it as a String.
// The number provided as diff corresponds to a difficulty option. See below.
pub fn choose_word(diff: u32) -> String {

    let filename = "src/dictionary.txt";
    if filename.contains("testdict") {
        println!("DEV NOTICE: You are using the test dictionary!")
    }

    //This function supports difficulty options! Depending on what is specified by diff,
    //this function will only return words of a certain (arbitrary) minimum and maximum length.
    /*
    Difficulty 0 - All words allowed.
    Difficulty 1 - Words of length 6 or less.
    Difficulty 2 - Words of length 6 to 9.
    Difficulty 3 - Words of length 9 or greater.
     */

    //Can I just say, I love the match statement over the switch statement in other languages.
    //If difficulty is easy or medium (1 or 2), max length gets set.
    //Otherwise, there isn't one.
    let max_length = match diff {
        1 => 6,
        2 => 9,
        _ => 0
    }; //for difficulties 0 and 3, there is no maximum length.

    let min_length = match diff {
        2 => 6,
        3 => 9,
        _ => 0
    }; //for difficulties 0 and 1, there is no minimum length.


    //We love OpenOptions!
    //Attempts to open the specified FilePath in read-only mode.
    //The .expect() is for error handling.
    let mut file = OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("ERROR: File not found!");

    let reader = BufReader::new(file); //Standard buffered reader


    //Okay, SO.
    //reader.lines() returns an iterator over the lines in the file - this iterator is a Lines object
    //This Lines iterator is....kinda useless. So, we use .map() to convert it into an iterator of Strings.
    //This String iterator (called 'lines') can then be used with .choose() and .expect() below.
    let lines = reader.lines().map(|l| l.expect("Couldn't read lines!"));

    //This loop is for only returning words of a specified length or lower.
    //Words will continue to be randomly chosen until one that fits the specified length is found.
    loop {
        let word_to_return = lines
            .choose(&mut rand::thread_rng())
            .expect("ERROR: Could not read lines of the file!");

        let word_length = u32::from(word_to_return.len());

        match diff {
            0 => return word_to_return,
            1 if (word_length <= max_length) => return word_to_return,
            2 if (word_length <= max_length && word_length >= min_length) => return word_to_return,
            3 if (word_length >= min_length) => return word_to_return,
            _ => return word_to_return
        }
    }

}

// Appends the word provided as a parameter to the dictionary file
pub fn add_word(word: &str) {

    let filename = "src/testdict.txt";

    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("ERROR: File not found!");
    writeln!(file, word).expect("ERROR: Could not write to file!");
}