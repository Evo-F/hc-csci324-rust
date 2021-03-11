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
* Variables are immutable (constants) by default, and must be declared mutable explicitly. 
    - Combination of C/C++ primitives and new Rust structs (which are also treated as primitives). Example: str vs. String
* Functions themselves have values, and the value of the last statement is returned implicitly. You **do not** need a return statement unless you want to return before the end of the function. 
    - Very similar to functional languages!
* Basic Rust philosophy is that the user should **never** see an error the developer didn't write themselves. As a result, many common **runtime** errors in other languages are elevated to **compile-time** errors in Rust.
    - For example, Rust will not allow you to accept user input without having some form of error handling. This is because even if the code was perfectly-written (which it never is), the user is still an unknown "x-factor" who could get the program into an unpredictable and unsafe state.
    - We discussed in class that compilers will sometimes **attempt recovery** after detecting a syntax error, which sometimes leads to catastrophic error-detection chains. The Rust compiler does not do this, instead failing after hitting the first syntax error. This is a bit frustrating, as it makes the development process very stop-and-go once it's time to compile, but it ensures that the developer only has to focus on one error at a time.
* Rust is fast! 
    - From Wikipedia: "Performance of idiomatic Rust is comparable to performance of equivalent idiomatic C++." 
    - Enough said really.
* The typical concept of "scopes" is replaced by "ownership". 
    - A function that creates data is said to "own" that data, *even if it gets returned.*
    - As such, the calling function needs to "borrow" that data.



