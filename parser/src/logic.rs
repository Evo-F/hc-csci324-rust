use std::collections::HashMap;

pub fn config_hashmaps(items: HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                       terms: HashMap<(String, i32), bool>,
                       input: String) -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                                          HashMap<(String, i32), bool>,
                                          String){

    let tokens = input.split_whitespace();
    let mut token_list = Vec::new();
    for token in tokens {
        token_list.push(token.to_string());
    }

    for str in &token_list {
        print!("{}", str);
        print!(", ");
    }
    println!();
    println!("TOKENS COLLECTED: {}", &token_list.len());
    println!();

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

    return (items, terms, input);

}