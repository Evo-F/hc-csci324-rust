use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn read_input() {
    let mut count = 0;
    let nonterms = ["&", "|", "!", "="];
    let mut expr_map = HashMap::new();

    print!("Enter Expression: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let expr = input.split_whitespace();

    for symbol in expr {
        if nonterms.contains(&symbol) {
            count = count + 1;
            match symbol {
                "&" => expr_map.insert((symbol.to_string(), count), (1, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                "|" => expr_map.insert((symbol.to_string(), count), (2, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                "!" => expr_map.insert((symbol.to_string(), count), (3, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                "=" => expr_map.insert((symbol.to_string(), count), (4, ("Sub1".to_string(), 0), ("Sub2".to_string(), 0))),
                _ => expr_map.insert((symbol.to_string(), count), (0, ("Null".to_string(), 0), ("Null".to_string(), 0)))
            };
        }
        else if !expr_map.contains_key(&(symbol.to_string(), 0)) {
            expr_map.insert((symbol.to_string(), 0), (0, ("Stuff".to_string(), 0), ("Things".to_string(), 0)));
        }

    }

    for (key, value) in &expr_map {
        println!("{:?} / {:?}", key, value);
    }

}