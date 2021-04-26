use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

mod tokenizer;
mod logic;

//test comment


fn main() {
    //We are going to test the parsing of "a AND b."
    //STILL WORKING ON THIS.
    /*
    let mut items: HashMap<(String, i32), (i32, (String, i32), (String, i32))> = HashMap::new();
    let mut terms: HashMap<(String, i32), bool> = HashMap::new();

    let a_key: (String, i32) = ("a".to_string(), 0);
    let b_key: (String, i32) = ("b".to_string(), 0);
    let and_key: (String, i32) = ("&".to_string(), 0);

    items.insert(a_key.clone(), (0, a_key.clone(), a_key.clone()));
    items.insert(b_key.clone(), (0, b_key.clone(), b_key.clone()));
    items.insert(and_key.clone(), (1, a_key.clone(), b_key.clone()));

    terms.insert(a_key.clone(), false);
    terms.insert(b_key.clone(), false);

    let eval_tuple = items.get(&and_key);
    if eval_tuple.is_none() {
        println!("Oops! Something went wrong!")
    } else {
        // EVAL 1 - Both false.
        let eval_tuple_real = eval_tuple.unwrap();

        println!("{}", evaluate(eval_tuple_real.clone(), &items, &terms));

        // EVAL 2 - Both true.
        terms.insert(a_key.clone(), true);
        terms.insert(b_key.clone(), true);
        println!("{}", evaluate(eval_tuple_real.clone(), &items, &terms));

        // EVAL 3 - A false, B true.
        terms.insert(a_key.clone(), false);
        println!("{}", evaluate(eval_tuple_real.clone(), &items, &terms));

        // EVAL 4 - A true, B false.
        terms.insert(a_key.clone(), true);
        terms.insert(b_key.clone(), false);
        println!("{}", evaluate(eval_tuple_real.clone(), &items, &terms));


    }*/

    let (mut a, mut b, c) = tokenizer::read_input();
    let (mut items, mut terms, input) = logic::config_hashmaps(a, b, c);

    // BELOW IS ALL DEBUG PRINTING
    /*
    println!("EXPR_MAP CONTENTS:");
    for (key, value) in &items {
        println!("{:?} / {:?}", key, value);
    }
    println!();
    println!("TERM_MAP CONTENTS:");
    for (key, value) in &terms {
        println!("{:?} / {:?}", key, value);
    }
    println!();
    println!("UNDERSTOOD TOKENS:");
    for piece in parsed_input {
        print!("{:?}", piece);
        print!(", ");
    }
    println!();

    //END OF DEBUG PRINTING BLOCK
    */


}

fn evaluate(tuple: (i32, (String, i32), (String, i32)),
            items: &HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
            terms: &HashMap<(String, i32), bool>) -> bool {
    let (mode, a, b) = tuple;

    let mut a_temp;
    let mut b_temp;

    return if mode == 0 {
        // This means we've reached a terminal and just need to get its value.
        // The b element is not needed, as terminals point to themselves in closed loops.
        let a_temp = terms.get(&a);

        if a_temp.is_none() {
            return false;
        }

        *a_temp.unwrap()
    } else {
        let mut a_bool = false;
        let mut b_bool = false;

        a_temp = items.get(&a);
        b_temp = items.get(&b);

        if a_temp.is_none() || b_temp.is_none() {
            return false;
        }

        a_bool = evaluate(a_temp.unwrap().clone(), &items, &terms);
        b_bool = evaluate(b_temp.unwrap().clone(), &items, &terms);

        match mode {
            1 => a_bool && b_bool, // AND
            2 => a_bool || b_bool, // OR
            3 => !a_bool,          // NOT
            4 => a_bool == b_bool, // EQUIVALENCE
            _ => a_bool
        }
    }
}