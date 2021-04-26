use std::collections::HashMap;

pub fn config_hashmaps(mut items: HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                       mut token_list: Vec<(String, i32)>) -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                                           (String, i32)){
    // TAKES IN ITEMS AND TOKEN_LIST
    // RETURNS ITEMS AND ROOT KEY

    let mut root = ("Init".to_string(), -1);

    /*
    println!("TOKENS:");
    for str in &token_list {
        print!("{:?}", str);
        print!(", ");
    }
    println!();

    println!("EXPR_MAP CONTENTS:");
    for (key, value) in &items {
        println!("{:?} / {:?} / {:?}", key, value, value.0);
    }
    println!();
    println!("TERM_MAP CONTENTS:");
    for (key, value) in &terms {
        println!("{:?} / {:?}", key, value);
    }
    println!();
     */

    let nonterms = ["&".to_string(), "|".to_string(), "!".to_string(), "=".to_string()];

    let mut starting_index = 0;
    while token_list.len() > 1 {
        let mut index = 0;

        for i in starting_index..token_list.len() {
            let token_tuple = &token_list[i];
            let token: String = token_tuple.0.clone();
            if nonterms.contains(&token) {
                //The object we're now dealing with is a logical operator. This means we need the terms on either side of it.

                let left_term_key: (String, i32) = token_list[i-1].clone();
                let right_term_key: (String, i32) = token_list[i+1].clone();

                let value = items.get(&token_tuple).unwrap().clone();

                items.insert(token_tuple.clone(), (value.0, left_term_key, right_term_key));
                index = i;
                break;
            }
        }

        token_list.remove(index - 1); // removes left term
        token_list.remove(index); // accounting for shift to the left, removes right term
        // token_list is now "collapsed" a little bit
        starting_index = index;
    }

    root = token_list[0].clone();


    /*
    println!();
    println!("WE ARE NOW PAST THE MAIN FOR LOOP AND REVIEWING THE HASHMAPS AND TOKEN LIST.");
    println!();
    println!("TOKENS:");
    for str in &token_list {
        print!("{:?}", str);
        print!(", ");
    }
    println!();

    println!("EXPR_MAP CONTENTS:");
    for (key, value) in &items {
        println!("{:?} / {:?} / {:?}", key, value, value.0);
    }
    println!();
    println!("TERM_MAP CONTENTS:");
    for (key, value) in &terms {
        println!("{:?} / {:?}", key, value);
    }
    println!();
    println!("ROOT KEY: {:?}", root);
     */



    return (items, root);

}