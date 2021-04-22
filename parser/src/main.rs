mod logic;

//test comment

fn main() {
    //We are going to test the parsing of "a AND b."
    let a = false;
    let b = false;
    let mut a_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: a,
        parent: nil
    };

    let mut b_log = logic::LogicObject {
        mode: 0, //TERMINAL
        subsidiaries: Vec::new(),
        root: b,
        parent: nil
    };

    let root_log = logic::LogicObject {
        mode: 1, //AND
        subsidiaries: vec![a_log, b_log],
        root: false,
        parent: nil
    };
    a_log.parent = &root_log;
    b_log.parent = &root_log;

    let result = logic::evaluate(&root_log);
    println!("{}", result); //should evaluate to false

}