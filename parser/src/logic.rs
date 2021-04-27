use std::collections::HashMap;

pub fn config_hashmaps(mut items: HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                       mut token_list: Vec<(String, i32)>) -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                                           (String, i32)){
    // TAKES IN ITEMS AND TOKEN_LIST
    // RETURNS ITEMS AND ROOT KEY

    let mut root = ("Init".to_string(), -1);

    let nonterms = ["&".to_string(), "|".to_string(), "!".to_string(), "=".to_string()];

    let mut start_index = 1;
    let mut end_index = token_list.len();
    while token_list.len() > 1 {
        let mut index = 0;

        let mut removal = false;
        for i in start_index..end_index {
            let token_tuple = &token_list[i];
            let token: String = token_tuple.0.clone();

            if nonterms.contains(&token) {
                //The object we're now dealing with is a logical operator. This means we need the terms on either side of it.

                let left_term_key: (String, i32) = token_list[i-1].clone();
                let right_term_key: (String, i32) = token_list[i+1].clone();

                if left_term_key.0.eq("(") && right_term_key.0.eq(")") {
                    index = i;
                    removal = true;
                    break;
                }

                match left_term_key.0.as_str() {
                    ")" => {
                        end_index = i - 1;
                        for x in end_index .. usize::MIN {
                            let parenth_token = &token_list[x];
                            if parenth_token.0.eq("(") && parenth_token.1 == left_term_key.1 {
                                start_index = x;
                                break;
                            }
                        }
                        break;
                    },
                    _ => ()
                };

                match right_term_key.0.as_str() {
                    "(" => {
                        start_index = i + 1;
                        for x in start_index .. token_list.len() {
                            let parenth_token = &token_list[x];
                            if parenth_token.0.eq(")") && parenth_token.1 == right_term_key.1 {
                                end_index = x;
                                break;
                            }
                        }
                        break;
                    },
                    _ => ()
                }

                let value = items.get(&token_tuple).unwrap().clone();

                items.insert(token_tuple.clone(), (value.0, left_term_key, right_term_key));
                index = i;
                removal = true;
                break;
            }
        }

        if removal {
            token_list.remove(index - 1); // removes left term
            token_list.remove(index); // accounting for shift to the left, removes right term
            // token_list is now "collapsed" a little bit
            start_index = 1;
        }
    }

    root = token_list[0].clone();

    return (items, root);

}
