use std::env;
use rusqlite::Connection;

#[allow(unused_variables)]
#[allow(dead_code)]
struct Birthday {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

#[allow(dead_code)]
impl Birthday {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn day(&self) -> &i32 {
        &self.day
    }
    pub fn month(&self) -> &i32 {
        &self.month
    }
    pub fn year(&self) -> &i32 {
        &self.year
    }

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

//cargo run -- new "aryan" 01 30 2007
pub fn new(argum: &[String]) {
    let b_struct = Birthday {
        name: argum[2].clone(),
        month: argum[3].parse::<i32>().unwrap(),
        day: argum[4].parse::<i32>().unwrap(),
        year: argum[5].parse::<i32>().unwrap(),
    };

    let conn = Connection::open("birth.db").expect("Failed to open database");

    conn.execute(
        "INSERT INTO birthdays (name, month, day, year) VALUES (?1, ?2, ?3, ?4)",
     (b_struct.name(), b_struct.month(), b_struct.day(), b_struct.year()),
    ).expect("Failed to insert birthday");

    println!("Added birthday for {}", b_struct.name());
}

pub fn list() {
    unimplemented!()
}


fn adding_to_db() -> Birthday {
    unimplemented!()
}
