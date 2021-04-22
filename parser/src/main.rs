mod logic;

//test comment

fn main() {
    //We are going to test the parsing of "a AND b."
    //STILL WORKING ON THIS.
    unsafe {
        let mut a_log = logic::LogicObject {
            mode: 0,
            subsidiaries: Vec::new(),
            root: false
        };

        let mut b_log = logic::LogicObject {
            mode: 0,
            subsidiaries: Vec::new(),
            root: false
        };

        let mut and_log = logic::LogicObject {
            mode: 0,
            subsidiaries: Vec::new(),
            root: false
        };

        print!("{}", logic::evaluate(&and_log));

    }



}