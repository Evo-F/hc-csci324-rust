use std::io;
use std::io::prelude::*;
use crate::dict_ops::add_word;

mod dict_ops;

// UPDATE GAME - Print the ASCII art of the hangman board and the current state of the word. Return true if solved.
fn update_game(word:&str, guess_list:&str, strikes:i8) -> bool {

    let mut solved = true; // initialize the win condition
    let ascii_array:[&str; 9] = [ // store the ASCII art for the game stages
        "_______\n|\n|\n|\n|\n|", // build the gallows
        "_______\n|    |\n|\n|\n|\n|", // hang the rope
        "_______\n|    |\n|   ( )\n|\n|\n|", // draw the head
        "_______\n|    |\n|   ( )\n|    |\n|\n|", // draw the torso
        "_______\n|    |\n|   ( )\n|   /|\n|\n|", // draw the left arm
        "_______\n|    |\n|   ( )\n|   /|\\\n|\n|", // draw the right arm
        "_______\n|    |\n|   ( )\n|   /|\\\n|   /\n|", // draw the left leg
        "_______\n|    |\n|   ( )\n|   /|\\\n|   / \\\n|", // draw the right leg
        "_______\n|    |\n|   (_)\n|   /|\\\n|   / \\\n|" // hang the hangman
    ];
    println!("{}", ascii_array[strikes as usize]); // print out the appropriate hangman stage

    print!("> ");
    for char in word.chars() { // print out the word so far
        if guess_list.contains(char) || !char.is_alphabetic() { // for each guessed or non-alphabetic character
            print!("{} ", char); // print out the character
        }
        else { // for all unknown characters
            print!("_ "); // print a blank space
            solved = false; // if there are any blank spaces, the game has not been won yet
        }
    }
    println!();

    if solved || strikes == 8 { // if no spaces are blank, or the max number of strikes is received - return
        return solved; // returns true if the puzzle is solved, false otherwise
    }

    let mut guess_list: Vec<char> = guess_list.chars().collect();
    guess_list.sort(); // sort and clean up the list of guessed letters
    guess_list.dedup();

    print!("Used Letters: ");
    for letter in guess_list { // and print out all guessed letters
        print!("{} ", letter);
    }
    println!();
    println!("Lives Remaining: {}", 8 - strikes); // print out lives remaining
    return solved;
}

fn menu_select() -> u32 {
    println!();
    println!("_____________");
    println!("|          |");
    println!("|  ( H A N G M A N )");
    println!("|");
    println!("| (1) Play Game");
    println!("| (2) Set Difficulty");
    println!("| (3) Add Custom Words");
    println!("| (4) Quit");
    println!("|");
    print!("Select an Option (1-4): ");
    return get_menu_input(4, "Select a Valid Option (1-4): ");
}

fn diff_select() -> u32 {
    println!();
    println!("D I F F I C U L T Y");
    println!("(1) Easy"); // word min length of 0 and max length of 5
    println!("(2) Medium"); // word min length of 6 and max length of 9
    println!("(3) Hard"); // word min length of 10+ and no max length
    println!("(4) Random"); // no min length or max length
    print!("Select a Difficulty (1-4): ");
    io::stdout().flush().unwrap();
    return get_menu_input(4, "Select a Valid Difficulty (1-4): ");
}

fn add_custom_word() {
    println!();
    print!("A D D  W O R D");
    loop {
        println!();
        print!("Enter your custom word: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        add_word(input);
        println!("Your word '{}' has been added to the dictionary.", input);
        println!("(1) Add Another Word");
        println!("(2) Return to Menu");
        print!("Select an option (1-2): ");
        match get_menu_input(2, "Select a Valid Option (1-2): "){
            1 => continue,
            _ => break
        }
    }
}

fn get_menu_input(num_options:u32, err_message:&str) -> u32 {
    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new(); // Initialize input variable.
        std::io::stdin().read_line(&mut input).unwrap(); // Retrieve user input string.

        if input.len() > 2 { // If the user entered anything longer than a single character, error and loop.
            print!("{}", err_message);
            continue;
        } // Else continue
        let input = input
            .chars()
            .next()
            .unwrap(); // Convert string to char.
        if !input.is_numeric() { // If the char is not numeric, error and loop.
            print!("{}", err_message);
            continue;
        } // Else continue
        let input = input.to_digit(10).unwrap(); // Convert char to digit.
        if input > num_options || input < 1 { // If the digit is outside of the range of options, error and loop.
            print!("{}", err_message);
            continue;
        } // Else continue and return the user input, converted into a type <u32> integer.
        return input;
    }
}

fn play_game(difficulty:u32) {

    loop { // GAME LOOP - Loop until player chooses to quit to main menu.

        let mut strikes:i8 = 0; // number of strikes, 8 strikes triggers loss condition
        let mut guess_list:String = "".to_owned(); // string of past player guesses
        let word:&str = &dict_ops::choose_word(difficulty).to_lowercase(); //choose_word() returns String, needs to be borrowed
        //let word:&str = "[dev]-001";
        //println!("[Dev] The word is: {}", word);

        update_game(word, &guess_list, strikes); // call function to print initial board

        loop { // INPUT LOOP - Continuously prompt user for input until the win/loss condition is met.

            // PROMPT USER INPUT
            print!("Your Guess: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.len() > 1 { // If the user input a string, ignore all but the first character.
                if word == input {
                    guess_list.push_str(&input);
                    println!("This is the correct word!");
                } else {
                    println!("This is not the correct word.");
                    strikes += 1;
                }
            } // end of if input.len() > 1
            else {
                let input = input
                    .to_ascii_lowercase()
                    .chars()
                    .next()
                    .unwrap(); // only use the first character
                // INPUT RESULTS - Update guess_list with new input and inform player of input correctness.
                if !input.is_alphabetic() { // if the character is not a letter, ignore it
                    println!("This is not a valid letter.");
                } else if guess_list.contains(input) { // if the character has already been guessed, ignore it
                    println!("You have already guessed this letter.");
                } else if word.contains(input) { // if the character is contained in the word
                    guess_list.push(input); // push it to the guess_list
                    //println!("This letter is part of the word.");
                } else { // if the character was an incorrect guess
                    guess_list.push(input); // push it to the guess list
                    strikes += 1; // and give player a strike for their mistake
                    //println!("This letter is not part of the word.");
                }
            } // end of else

            // GAME RESULTS - End game if player has max number of strikes, or if the word has been solved.
            if update_game(word, &guess_list, strikes) {
                println!("The man is saved! You guessed the word.");
                break;
            } else if strikes >= 8 {
                println!("The man is hanged! The correct word was '{}'.", word);
                break;
            }
        } // end of input loop

        // PLAY AGAIN?
        println!("(1) Play Again");
        println!("(2) Return to Menu");
        print!("Select an Option (1-2): ");
        match get_menu_input(2, "Select a Valid Option (1-2): ") {
            1 => continue, // restarts game with current settings
            _ => break // leaves play_game() and returns to menu
        }
    } // end of game loop
}

fn main() {

    let mut difficulty:u32 = 4; // difficulty set to "Random" by default

        loop { // MENU LOOP - Player stays here until they select (4) Quit
            match menu_select() { // menu_select() returns an int between 1 and 4
                1 => play_game(difficulty), // start game with current settings, returns when game ends
                2 => difficulty = diff_select(), // select difficulty, returns when diff selected
                3 => add_custom_word(), // adds custom word to dictionary, returns when user backs out
                _ => break // quits game, ends main function
            }
        } // end of menu loop
    println!();
    println!("Thanks for playing!"); // the user must have quit
}