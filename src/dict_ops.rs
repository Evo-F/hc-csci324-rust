//TODO: This file contains code that was borrowed from this link:
//               https://stackoverflow.com/questions/50788009/how-do-i-get-a-random-line-from-a-file
// This code is present solely for testing. It needs to be modified and properly cited.

use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Randomly selects a word from the filepath and returns it as a String.
pub fn choose_word() -> String {

    let filename = "src/testdict.txt";
    if filename.contains("testdict") {
        println!("DEV NOTICE: You are using the test dictionary!")
    }
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.expect("Couldn't read lines!"));
    lines
        .choose(&mut rand::thread_rng())
        .expect("Issue with lines!")

}