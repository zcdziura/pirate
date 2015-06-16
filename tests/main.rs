extern crate pirate;

use std::env;

use pirate::{matches, vars};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let opts = vec!["o/opt(An option)", "a(Argument):"];
    let mut vars = vars("A Test Program", &opts).unwrap();
    
    let matches = match matches(&mut vars, &env_args) {
        Ok(m) => m,
        Err(why) => panic!("An error occurred: {}", why)
    };
    
    if matches.has_match("a") {
        let m = matches.get("a").unwrap();
        println!("{}", m);
    }
    
    match matches.get("opt") {
        Some(_) => println!("Opt was passed to the program"),
        None => println!("Opt was not passed to the program")
    }
}
