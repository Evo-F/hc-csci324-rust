use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::SplitWhitespace;

pub fn read_input() -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>, HashMap<(String, i32), bool>, Vec<(String, i32)>){
    let mut count = 0;
    let mut tokens = Vec::new();

    let nonterms = ["&", "|", "!", "=", "(", "[", ")", "]"];
    let logterms = ["&", "|", "!", "="];
    let mut expr_map = HashMap::new();
    let mut term_map = HashMap::new();

    loop {
        print!("Enter Expression: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let expr = input.split_whitespace();

        let mut parenth_score = 0;
        let mut index = 0;
        let mut valid_input = true;
        let mut symbol_vector = Vec::new();

        for symbol in expr.clone() {
            symbol_vector.push(symbol.clone());
        }

        for symbol in symbol_vector.clone() {
            valid_input = true;

            if logterms.contains(&symbol) && !symbol.eq("!") {
                let max_index = (symbol_vector.len() - 1) as i32;

                if index == 0 || index == max_index {
                    valid_input = false;
                }
            }

            match symbol {
                "(" | "[" => parenth_score += 1,
                ")" | "]" => parenth_score -= 1,
                _ => ()
            }
            if parenth_score < 0 {
                valid_input = false;
            }

            if !valid_input {
                break;
            }
            index += 1;
        }
        if !valid_input {
            println!("Invalid Input!");
            continue;
        }

        let mut parenth_depth = 0;

        for symbol in expr {
            if nonterms.contains(&symbol) {

                if symbol.eq("(") || symbol.eq("[") {
                    parenth_depth += 1;
                }

                match symbol {
                    "&" => expr_map.insert((symbol.to_string(), count), (1, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "|" => expr_map.insert((symbol.to_string(), count), (2, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "!" => expr_map.insert((symbol.to_string(), count), (3, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "=" => expr_map.insert((symbol.to_string(), count), (4, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "(" | "[" | ")" | "]" => expr_map.insert((symbol.to_string(), parenth_depth), (0, ("Null".to_string(), 0), ("Null".to_string(), 0))),
                    _ => expr_map.insert((symbol.to_string(), count), (0, ("Null".to_string(), 0), ("Null".to_string(), 0)))
                };

                if symbol.eq(")") || symbol.eq("]") {
                    parenth_depth -= 1;
                }

                tokens.push((symbol.to_string(), count));
            }
            else if !expr_map.contains_key(&(symbol.to_string(), 0)) {
                expr_map.insert((symbol.to_string(), 0), (0, ("Null".to_string(), 0), ("Null".to_string(), 0)));
                term_map.insert((symbol.to_string(), 0), false);
                tokens.push((symbol.to_string(), 0));
            }
            count = count + 1;

        }

        break;
    }


    return (expr_map, term_map, tokens);

}