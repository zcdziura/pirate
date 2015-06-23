extern crate pirate;

use std::env;

use pirate::{Matches, MatchesTrait, matches, usage, vars};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let opts = vec!["o/opt(An option)", "a(An Argument):"];
    let mut vars = vars("test", &opts).unwrap();
    
    let matches: Matches = match matches(&mut vars, &env_args) {
        Ok(m) => m,
        Err(why) => panic!("An error occurred: {}", why)
    };
    
    if matches.has_match("help") {
        usage(&vars);
        return;
    }

    let arg = match matches.get("a") {
        Some(a) => a.clone(),
        None => String::from("Pickle")
    };

    match matches.get("opt") {
        Some(_) => println!("Opt was passed to the program"),
        None => println!("Opt was not passed to the program")
    }
    
    println!("{}", arg);
}
