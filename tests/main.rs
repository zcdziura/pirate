extern crate pirate;

use std::env;

use pirate::Matches;

fn main() {
    let opts = vec!["n:", "b/boop", ":input"];

    let matches: Matches = match pirate::parse(env::args(), &opts) {
        Err(ref e) => {
            println!("Error: {}", e);
            help();
            return;
        },
        Ok(m) => m
    };

    // Print the program help if necessary
    if matches.has_arg("h") || matches.has_arg("help") {
        help();
        return;
    }

    let input = matches.get("input").unwrap().parse::<i32>().unwrap();

    let num = match matches.get("n") {
        Some(n) => n.parse::<i32>().unwrap(),
        None => 1
    };

    let sum = input + num; 

    println!("{} + {} = {}", input, num, sum); 

    if matches.has_arg("b") || matches.has_arg("boop") {
        println!("Boop!!");
    }
}

fn help() {
    println!("usage: pirate-test [-n NUM] [-b|--boop] INPUT\n");

    println!("Options:");
    println!("    -n NUM\tChange the default number that's added to the input");
    println!("    -b, --boop\tIt's a surprise!");

    println!("\nRequired arguments:");
    println!("    INPUT\tWe're gonna manipulate this somehow, you'll see!");
}
