use std::io;
use std::io::prelude::*;
mod dict_ops;

// UPDATE GAME - Print the ASCII art of the hangman board and the current state of the word. Return true if solved.
fn update_game(word:&str, guess_list:&str, strikes:i8)->bool {

    let mut solved = true; // initialize the win condition
    let ascii_array:[&str; 9] = ["_______\n|\n|\n|\n|\n|", // build the gallows
        "_______\n|    |\n|\n|\n|\n|", // hang the rope
        "_______\n|    |\n|   ( )\n|\n|\n|", // draw the head
        "_______\n|    |\n|   ( )\n|    |\n|\n|", // draw the torso
        "_______\n|    |\n|   ( )\n|   /|\n|\n|", // draw the left arm
        "_______\n|    |\n|   ( )\n|   /|\\\n|\n|", // draw the right arm
        "_______\n|    |\n|   ( )\n|   /|\\\n|   /\n|", // draw the left leg
        "_______\n|    |\n|   ( )\n|   /|\\\n|   / \\\n|", // draw the right leg
        "_______\n|    |\n|   (_)\n|   /|\\\n|   / \\\n|"]; // hang the hangman
    println!("{}", ascii_array[strikes as usize]); // print out the appropriate hangman stage
    print!("> ");
    for char in word.chars() { // print out the word so far
        if guess_list.contains(char) { // for each character that has been previously guessed
            print!("{} ", char); // print out the characters that have been guessed
        }
        else { // for all unknown characters
            print!("_ "); // print a blank space
            solved = false; // if there are any blank spaces, the game has not been won yet
        }
    }
    println!(); // if no spaces are blank, the word is solved - return true
    return solved; // else return false
}


fn main() {

    // GAME LOOP - Continuously reset the game until the user wishes to stop.
    loop {

        /*print!("1 - Easy\n2 - Medium\n3 - Hard\nSelect a difficulty (1-3): ");
        io::stdout().flush().unwrap();
        let mut diff = String::new();
        std::io::stdin().read_line(&mut diff).unwrap();
        let diff = diff.trim().parse::<u32>().unwrap();*/

        // INITIALIZE ALL VARIABLES
        let mut strikes:i8 = 0; // number of strikes, 7 strikes triggers loss condition
        let mut guess_list:String = "".to_owned(); // string of past player guesses
        let word:&str = &dict_ops::choose_word(1).to_lowercase(); //choose_word() returns String, needs to be borrowed
        //let dev_mode = true;
        //if dev_mode {
        //    println!("DEV MODE IS ENABLED. DEV MESSAGES WILL BE PRINTED.");
        //    println!("THE WORD IS: {}", word);
        //}
        update_game(word, &guess_list, strikes); // call function to print initial board

        // INPUT LOOP - Continuously prompt user for input until the win/loss condition is met.
        loop {

            // PROMPT USER INPUT
            print!("Enter your guess: ");
            io::stdout().flush().unwrap();
            let mut guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            let guess = guess.trim();

            if guess.len() > 1 { // If the user input a string, ignore all but the first character.
                if word == guess {
                    guess_list.push_str(&guess);
                    println!("'{}' is the correct word!", guess);
                } else {
                    println!("'{}' is not the correct word.", guess);
                    strikes += 1;
                }
            } // end of if guess.len() > 1
            else {
                let guess = guess.to_ascii_lowercase().chars().next().unwrap(); // only use the first character

                // INPUT RESULTS - Update guess_list with new input and inform player of input correctness.
                if !guess.is_alphabetic() { // if the character is not a letter, ignore it
                    println!("'{}' is not a valid letter.", guess);
                } else if guess_list.contains(guess) { // if the character has already been guessed, ignore it
                    println!("'{}' has already been guessed.", guess);
                } else if word.contains(guess) { // if the character is contained in the word
                    guess_list.push(guess); // push it to the guess_list
                    println!("'{}' was a correct guess.", guess);
                } else { // if the character was an incorrect guess
                    guess_list.push(guess); // push it to the guess list
                    strikes += 1; // and give player a strike for their mistake
                    println!("'{}' was not a correct guess.", guess);
                }
            } // end of else

            // GAME RESULTS - End game if player has max number of strikes, or if the word has been solved.
            if update_game(word, &guess_list, strikes) {
                println!("THE MAN IS SAVED");
                break;
            } else if strikes >= 8 {
                println!("THE MAN IS HANGED\nThe correct word was '{}'", word);
                break;
            }
        } // end of input loop

        print!("Play Again? (y/n) ");
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).unwrap();
        let play_again = play_again.trim();
        if play_again == "y" {
            println!("Here we go again!");
            continue;
        }
        else {
            println!("Thanks for playing!");
            break;
        }

    } // end of game loop
}