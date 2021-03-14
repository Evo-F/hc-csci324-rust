//TODO: This file contains code that was borrowed from this link:
//               https://stackoverflow.com/questions/50788009/how-do-i-get-a-random-line-from-a-file
// This code is present solely for testing. It needs to be modified and properly cited.

use rand::seq::IteratorRandom;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

// Randomly selects a word from the filepath and returns it as a String.
pub fn choose_word() -> String {

    let filename = "src/dictionary.txt";
    if filename.contains("testdict") {
        println!("DEV NOTICE: You are using the test dictionary!")
    }

    //We love OpenOptions!
    //Attempts to open the specified FilePath in read-only mode.
    //The .expect() is for error handling.
    let mut file = OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("ERROR: File not found!");

    let reader = BufReader::new(file); //Standard buffered reader

    let lines = reader.lines().map(|l| l.expect("Couldn't read lines!"));
    //Okay, SO.
    //reader.lines() returns an iterator over the lines in the file - this iterator is a Lines object
    //This Lines iterator is....kinda useless. So, we use .map() to convert it into an iterator of Strings.
    //This String iterator (called 'lines') can then be used with .choose() and .expect() below.
     lines
        .choose(&mut rand::thread_rng())
        .expect("Issue with lines!")
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