//mod logic;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


//test comment
fn check_syntax(expr:String) {

}

fn read_input() {
    let mut count = 1;
    let nonterms = ["&", "|", "!", "="];
    let mut expr_map = HashMap::new();

    print!("Enter something: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let expr = input.split_whitespace();

    // do syntax checking
    // check_syntax(input);

    for symbol in expr {
        // if the symbol is a nonterminal, identify it and insert it into the map
        if nonterms.contains(&symbol) {
            match symbol {
                "&" => expr_map.insert((symbol, count), "AND"),
                "|" => expr_map.insert((symbol, count), "OR"),
                "!" => expr_map.insert((symbol, count), "NOT"),
                "=" => expr_map.insert((symbol, count), "EQUAL"),
                _ => expr_map.insert((symbol, count), "ERROR")
            };
            count = count + 1;
        }
        else {
            // if the terminal symbol doesn't exist, insert it into the map
            if !expr_map.contains_key(&(symbol, 0)) {
                expr_map.insert((symbol, 0), "TERMINAL");
            }
        }
    }
    for (symbol, value) in &expr_map {
        println!("{:?}: {}", symbol, value);
    }
}

fn main() {
    //We are going to test the parsing of "a AND b."
    let a = false;
    let b = false;
    /*let mut a_log = logic::LogicObject {
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
    */
    read_input();

}