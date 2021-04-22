#[std::]
pub struct LogicObject<'a> {
    pub mode: u32,
    pub subsidiaries: Vec<&'a LogicObject<'a>>,
    pub root: bool
} /*
How does the LogicObject structure work?
Well, it stores a representation of a logical expression, controlled by its operator.

The mode attribute is an integer and represents how the LogicObject is working. It can have the following values:
-1 - The LogicObject is in SINGLETON mode. When evaluated, it will return the boolean value of the first element in 'subsidiaries'.
        //This is the default case.
0 - The LogicObject is in TERMINAL mode. When evaluated, it will return the boolean value of its 'root' attribute.
1 - The LogicObject is in AND mode. When evaluated, it will return the boolean value of 'a && b && c...' where a/b/c represent the LogicObjects stored in 'subsidiaries'.
2 - The LogicObject is in OR mode. See note for AND mode, but replace && with ||.
3 - The LogicObject is in NOT mode. When evaluated, it will return the inverted boolean value of the first element in 'subsidiaries'.
4 - The LogicObject is in EQUIVALENCE mode. When evaluated, it will return the value of 'a == b == c...'.

The 'subsidiaries' attribute is a vector (collection) of other LogicObjects that are connected to this one.
This allows us to create branching logical trees from an expression.

The 'parent' attribute points to the LogicObject which is "holding" this LogicObject, if applicable.

The 'root' attribute is a reference to a boolean, and should only be used if the LogicObject is in TERMINAL mode.

*/

pub fn evaluate(obj: &LogicObject) -> bool {
    return match obj.mode {
        0 => obj.root,
        1 => and(obj),
        2 => or(obj),
        3 => not(obj),
        4 => equiv(obj),
        _ => evaluate(&obj.subsidiaries[0])
    }
}

fn and(obj: &LogicObject) -> bool {
    let mut result_and = true;
    for o in &obj.subsidiaries {
        result_and = result_and && evaluate(o);
    }
    return result_and;
}

fn or(obj: &LogicObject) -> bool {
    let mut result_or = false;
    for o in &obj.subsidiaries {
        result_or = result_or || evaluate(o);
    }
    return result_or;
}

fn not(obj: &LogicObject) -> bool {
    return !(evaluate(&obj.subsidiaries[0]));
}

fn equiv(obj: &LogicObject) -> bool {
    let mut result_equiv = evaluate(&obj.subsidiaries[0]);
    for o in &obj.subsidiaries {
        result_equiv = result_equiv == evaluate(o);
    }

    return result_equiv;
}

