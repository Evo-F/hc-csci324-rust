# hc-csci324-rust
NAMES: Evo Fearnley (2021), Justin Bella (2021)

This repository is for all files related to our term project for CSCI 324 (Programming Languages: Design and Implementation). 

## Roadmap/Plans
### Common Program (Hangman) - due 3/15
* Evo will be handling the backend of the common program. This includes all file read/write operations, the selection of the word itself, and the verification of user guesses.
* Justin will be handling the frontend of the common program. This includes the user interface itself, the collection of user input, and visual presentation of feedback. 
* Because we do not have a native graphical interface, the program should allow users to add their own words to the file seamlessly. 
* The program should also have user-configurable behavior where possible and reasonable - for example, a difficulty setting that impacts the length of words chosen.
* If time allows, we should also supply a secondary dictionary file of phrases and sentences, with punctuation allowed but not user-guessable (it appears by default). 

### Creative Program (TBD)
### Paper (TBD)
### Presentation (TBD)
#### Areas of Focus
* Rust is a **very safe** language and takes steps to prevent race conditions, memory corruption, null pointer exceptions, and more.
* Basic Rust philosophy is that the user should **never** see an error the developer didn't write themselves. As a result, many common **runtime** errors in other languages are elevated to **compile-time** errors in Rust.
    - For example, Rust will not allow you to accept user input without having some form of error handling. This is because even if the code was perfectly-written (which it never is), the user is still an unknown "x-factor" who could get the program into an unpredictable and unsafe state.
* Rust is fast! 
    - From Wikipedia: "Performance of idiomatic Rust is comparable to performance of equivalent idiomatic C++." 
    - Enough said really.



