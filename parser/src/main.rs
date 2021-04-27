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

    let (mut a, mut terms, c) = tokenizer::read_input();
    /*
    A - The items map.
    B - The terms map.
    C - The tokenized vector.
     */
    let (items, root) = logic::config_hashmaps(a, c);



    println!("RESULT 1: {}", evaluate(root.clone(), &items, &terms));
    terms.insert(("a".to_string(), 0), true);
    terms.insert(("b".to_string(), 0), true);
    println!("RESULT 2: {}", evaluate(root.clone(), &items, &terms));



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

fn evaluate(key: (String, i32),
            items: &HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
            terms: &HashMap<(String, i32), bool>) -> bool {

    let (mode, a, b) = items.get(&key).unwrap();

    return if *mode == 0 {
        // This means we've reached a terminal and just need to get its value.
        // The b element is not needed, as terminals point to themselves in closed loops.
        let a_temp = terms.get(&key);

        if a_temp.is_none() {
            return false;
        }

        *a_temp.unwrap()
    } else {
        let mut a_bool = false;
        let mut b_bool = false;

        a_bool = evaluate(a.clone(), &items, &terms);
        b_bool = evaluate(b.clone(), &items, &terms);

        match mode {
            1 => a_bool && b_bool, // AND
            2 => a_bool || b_bool, // OR
            3 => !a_bool,          // NOT
            4 => a_bool == b_bool, // EQUIVALENCE
            _ => a_bool
        }
    }
}