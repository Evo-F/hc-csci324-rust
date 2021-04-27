use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::str::SplitWhitespace;

pub fn read_input() -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>, HashMap<(String, i32), bool>, Vec<(String, i32)>){
    let mut count = 0;
    let mut tokens = Vec::new();
    //let mut parentheses_ver = 0;
    let nonterms = ["&", "|", "!", "=", "(", ")"];
    let mut expr_map = HashMap::new();
    let mut term_map = HashMap::new();

    loop {
        print!("Enter Expression: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let expr = input.split_whitespace();
        let mut parenth_count = 0;

        for symbol in expr.clone() {
            match symbol {
                "(" | "[" => parenth_count = parenth_count + 1,
                ")" | "]" => parenth_count = parenth_count - 1,
                _ => continue
            }
            if parenth_count < 0 {
                break;
            }
        }

        if parenth_count != 0 {
            println!("Invalid input! Please try again!");
            continue;
        }

        let mut parenth_depth = 0;
        for symbol in expr {
            if nonterms.contains(&symbol) {
                if symbol.eq("(") || symbol.eq("[") {
                    parenth_depth = parenth_depth + 1;
                }
                if symbol.eq(")") || symbol.eq("]") {
                    parenth_depth = parenth_depth - 1;
                }

                match symbol {
                    "&" => expr_map.insert((symbol.to_string(), count), (1, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "|" => expr_map.insert((symbol.to_string(), count), (2, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "!" => expr_map.insert((symbol.to_string(), count), (3, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "=" => expr_map.insert((symbol.to_string(), count), (4, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                    "(" | "[" => expr_map.insert((symbol.to_string(), parenth_depth), (0, ("Null".to_string(), 0), ("Null".to_string(), 0))),
                    ")" | "]" => expr_map.insert((symbol.to_string(), parenth_depth), (0, ("Null".to_string(), 0), ("Null".to_string(), 0))),
                    _ => expr_map.insert((symbol.to_string(), count), (0, ("Null".to_string(), 0), ("Null".to_string(), 0)))
                };
                tokens.push((symbol.to_string(), count));
            }
            else if !expr_map.contains_key(&(symbol.to_string(), 0)) {
                expr_map.insert((symbol.to_string(), 0), (0, ("Stuff".to_string(), 0), ("Things".to_string(), 0)));
                term_map.insert((symbol.to_string(), 0), false);
                tokens.push((symbol.to_string(), 0));
            }
            count = count + 1;

        }

        break;
    }



    return (expr_map, term_map, tokens);

}