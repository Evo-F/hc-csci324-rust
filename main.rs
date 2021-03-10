use std::io;

fn game_update(word:&str, guess_list:&str, strikes:i32)->bool{
    return true;
    return false;
}

fn main() {
    let mut solved = false;
    let mut strikes = 0;              // number of strikes player has
    let mut guesses = "";           // string of past guesses
    let word = "wrath";         // the word - retrieved from the dictionary
    println!("Here's your word:");   // print out number of blanks in the word
    for _c in 0..word.len() {
        print!("_ ");
    }

    while solved != true {
        println!("\nEnter your guess: ");

        let mut next_guess = String::new();
        std::io::stdin().read_line(&mut next_guess).unwrap();
        let next_guess = next_guess.chars().next().unwrap();
        println!("You guessed: '{}'", next_guess);

        if guesses.contains(next_guess) {
            println!("You've already guessed that letter.");
        } else if word.contains(next_guess) {
            println!("Good guess!");
        } else {
            println!("That's not correct.");
            strikes += 1;
        }
        let guesses = concat!(guesses.to_string(), next_guess.to_string());
        println!("{}", guesses);
    }
/*
    while guess.chars().count() != 1 {
        // println!("top {}", guess.chars().count());
        println!("You may only enter one letter at a time. Try again!");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        // println!("bot {}", guess.chars().count());
    }
*/
}
