mod logic;

fn main() {
    //We are going to test the parsing of "a AND b."
    let a = false;
    let b = false;
    let a_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: &a,
    };

    let b_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: &b,
    };

    let root_log = logic::LogicObject {
        mode: 1, //AND
        subsidiaries: vec![a_log, b_log],
        root: &false,
    };

    let result = logic::evaluate(&root_log);
    println!("{}", result); //should evaluate to false

}