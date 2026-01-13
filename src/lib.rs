use std::{env};

#[allow(unused_variables)]
#[allow(dead_code)]
struct Birthday {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

impl Birthday {

}
pub fn match_functions() {

    let args: Vec<String> = env::args().collect();
    let query  = args[1].as_str();
   
 if args.len() >= 2 {
    match query {
        "new"=> new(&args),
        "get" => list(),
        _ => println!("Unknown command"),
        }
    }
}

// cargo run -- new "aryan" 01 30 2007
// TODO: Figure out how to insert the struct variables into sqlite, also increment id
pub fn new(argum: &[String]) {
    let b_struct = Birthday {
        name: argum[2].clone(),
        day: argum[3].parse::<i32>().unwrap(),
        month: argum[4].parse::<i32>().unwrap(),
        year: argum[5].parse::<i32>().unwrap(), 
    };

}

//TODO: implement this after everything is said and done
pub fn list() {
    unimplemented!()
}



fn adding_to_db() -> Birthday {
    unimplemented!()
}
