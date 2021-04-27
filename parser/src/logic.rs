use std::collections::HashMap;

pub fn config_hashmaps(mut items: HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                       mut token_list: Vec<(String, i32)>) -> (HashMap<(String, i32), (i32, (String, i32), (String, i32))>,
                                           (String, i32)){
    // TAKES IN ITEMS AND TOKEN_LIST
    // RETURNS ITEMS AND ROOT KEY

    let mut root = ("Init".to_string(), -1);

    let nonterms = ["&".to_string(), "|".to_string(), "!".to_string(), "=".to_string()];

    let mut start_index = 0;

    let mut end_index = token_list.len() - 1;

    let mut pass_type = 0;

    while token_list.len() > 1 {
        let mut index = 0;
        let max_index = token_list.len() - 1;

        let mut l_removal = false;
        let mut r_removal = false;

        // 0: PARENTHETICAL PASS
        let mut parenth_index = 0;
        let mut parenth_id = 0;
        if pass_type == 0 {
            for (token, num) in token_list.clone() {
                if token.eq("(") && num > parenth_id {
                    parenth_id = num;
                    start_index = parenth_index;
                }

                if token.eq(")") && num == parenth_id {
                    end_index = parenth_index;
                    break;
                }
                parenth_index += 1;
            }
            if parenth_id == 0 {
                // All parentheses have been resolved properly!
                // Set bounding indices to the entire token list, then advance to next level of metaprecedence.
                // This way, we don't keep repeating the same loop for no reason.
                start_index = 0;
                end_index = max_index;
                pass_type = 1;

            }
        }

        // 1: EQUALITY PASS




        // FINAL: PARSE PASS
        for i in start_index..end_index {
            if end_index > max_index {
                // We've got a weird overflow here, so we need to step it back to resolve it.
                end_index = end_index - 1;
                break;
            }
            let token_tuple = &token_list[i];
            let token: String = token_tuple.0.clone();

            if nonterms.contains(&token) {
                //The object we're now dealing with is a logical operator. This means we need the terms on either side of it.

                let value = items.get(&token_tuple).unwrap().clone();

                let mut left_term_key;
                let mut right_term_key;

                if i == 0 {
                    left_term_key = token_list[i].clone();
                } else {
                    left_term_key = token_list[i-1].clone();
                }

                if i == max_index {
                    right_term_key = token_list[i].clone();
                } else {
                    right_term_key = token_list[i+1].clone();
                }



                if left_term_key.0.eq("(") && right_term_key.0.eq(")") {
                    index = i;
                    l_removal = true;
                    r_removal = true;
                    break;
                }

                if token.eq("!") {
                    if value.1 == value.2 {
                        continue;
                        // If we encounter a ! symbol, and value.1 and value.2 are equal,
                        // that means we've already parsed this ! symbol and SHOULD NOT DO SO AGAIN.
                        // Proceed to the next symbol.
                    }

                    index = i;
                    r_removal = true;
                    items.insert(token_tuple.clone(), (value.0, right_term_key.clone(), right_term_key.clone()));
                    break;
                } else if i == 0 || i == max_index {
                    // We somehow have a dangling logical operator.
                    // This should never happen.
                    continue;
                } else if token.eq("&") || token.eq("|") {
                    items.insert(token_tuple.clone(), (value.0, left_term_key, right_term_key));
                    index = i;
                    l_removal = true;
                    r_removal = true;
                    break;
                } else if token.eq("=") {
                    // Because EQUALITY gets evaluated last, everything on either side of it needs to be fully resolved.
                    if left_term_key.1 > 0 {
                        // The term on the left is a logical expression.
                        // Make sure it's already been evaluated.
                        let left_term_value = items.get(&left_term_key.clone()).unwrap().clone();
                        if left_term_value.1.0.eq("Sub1") || left_term_value.2.0.eq("Sub2") {
                            // WE HAVE NOT YET EVALUATED THE LEFTHAND EXPRESSION.
                            // KEEP ON GOING.
                            continue;
                        }
                    }
                    if right_term_key.1 > 0 {
                        // The term on the right is a logical expression.
                        // Make sure it's already been evaluated.
                        let right_term_value = items.get(&right_term_key.clone()).unwrap().clone();
                        if right_term_value.1.0.eq("Sub1") || right_term_value.2.0.eq("Sub2") {
                            // WE HAVE NOT YET EVALUATED THE RIGHTHAND EXPRESSION.
                            // KEEP ON GOING.
                            continue;
                        }
                    }

                    // If we've made it here, then both lefthand and righthand terms are terminals
                    // or they are fully-evaluated expressions.
                    // We can do our thing as per usual.
                    items.insert(token_tuple.clone(), (value.0, left_term_key, right_term_key));
                    index = i;
                    l_removal = true;
                    r_removal = true;
                    break;

                }  else {
                    // We have somehow encountered a valid logical symbol that isn't explicitly covered above.
                    // Currently these are: none.
                    // So....we should never be here.
                    items.insert(token_tuple.clone(), (value.0, left_term_key, right_term_key));
                    index = i;
                    l_removal = true;
                    r_removal = true;
                    break;
                }
            }
        }

        if index < token_list.len() {
            if l_removal && r_removal {
                token_list.remove(index - 1); // removes left term
                token_list.remove(index); // accounting for shift to the left, removes right term
                // token_list is now "collapsed" a little bit
                start_index = 0;
            } else if l_removal {
                token_list.remove(index - 1);
                start_index = 0;
            } else if r_removal {
                token_list.remove(index + 1);
                start_index = 0;
            }

        }

    }

    root = token_list[0].clone();

    return (items, root);

}
