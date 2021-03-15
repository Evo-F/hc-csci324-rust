# hc-csci324-rust
NAMES: Evo Fearnley (2021), Justin Bella (2021)

This repository is for all files related to our term project for CSCI 324 (Programming Languages: Design and Implementation). 

## Roadmap/Plans
### Common Program (Hangman) - due 3/15 - Complete!
* Evo will be handling the backend of the common program. This includes all file read/write operations, the selection of the word itself, and the verification of user guesses.
* Justin will be handling the frontend of the common program. This includes the user interface itself, the collection of user input, and visual presentation of feedback. 
* Because we do not have a native graphical interface, the program should allow users to add their own words to the file seamlessly. 
* The program should also have user-configurable behavior where possible and reasonable - for example, a difficulty setting that impacts the length of words chosen.
* If the user enters multiple characters at once, they are treated as a guess at the word. If that guess is incorrect, the hanged man gains an additional limb.
    - May later add a user-configurable option to treat string entries as sequential character entries. For example, if the word is "elephant" and the user enters "phan", it would be viewed as the user guessing p, h, a, and n in sequence. 

### Creative Program (Working Idea - Logic Parser)
* The program should be able to parse logical expressions in standard boolean format.
* The program will first generate a truth table for the expression. This truth table will be displayed to the user. 
    - NOTE: Because the truth table has 2^n rows, where n is the number of logical symbols present, the expression entered will need to be limited in how many symbols it can include, for performance reasons.
* The program will then **simplify the logical expression** as much as possible, based on the principle that expressions with identical truth tables are equivalent. 
    - This might mean that some logical symbols get discarded entirely, if the program determines that they have no bearing on the result of the overall expression.
    - For example, the expression (a OR (a AND b AND c)) can be simplified to just (a), as the values of (b) and (c) are meaningless.
    - The program should do this by determining the correlation between each value and the overall value of the expression according to the truth table. In the above example, the value of (a) has a 100% positive correlation with the final result - if the expression is true, (a) is ALWAYS true. 
    - By definition, at least for our purposes, a shorter logical expression is simpler. Therefore, if the program derives a "simplified" expression from the truth table which is **longer** than the original expression, the original is treated as simplified. 
* Basic Correlation Rules
    - 100% positive - If expression is true, value (a) is ALWAYS true. Expression can be reduced to (a).
    - 100% negative - If expression is true, value (a) is ALWAYS false. Expression can be reduced to (NOT a). 
* Supported Logical Operations
    - Logical Symbols + Raw Boolean Values (true/false)
    - Conjunction (AND/&)
    - Disjunction (OR/|)
    - Inequality (XOR/^)
    - Inverse (NOT/!)
    - Parentheses
    - Equivalence (=)
    - Implication (>) 
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
* Functions can consume their parent! 
    - Certain functions which require a `self` parameter can essentially delete their parent structure when run, which is fantastic for memory efficiency and performance but absolutely terrifying if you need to do anything in a loop.
    - For example: The `iterator.choose()` function (standard random selection from iterator). This function will consume the `iterator` parent object, so if you're trying to `.choose()` in a loop, you're going to have a bad time.



