mod logic;
use std::io;
use std::io::prelude::*;


//test comment

fn read_input() {

    print!("Enter something: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();


}

fn main() {
    //We are going to test the parsing of "a AND b."
    let a = false;
    let b = false;
    let mut a_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: a,
        parent: nil
    };

    let mut b_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: b,
        parent: nil
    };

    let root_log = logic::LogicObject {
        mode: 1, //AND
        subsidiaries: vec![a_log, b_log],
        root: false,
        parent: nil
    };
    a_log.parent = &root_log;
    b_log.parent = &root_log;

    let result = logic::evaluate(&root_log);
    println!("{}", result); //should evaluate to false

}