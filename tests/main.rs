extern crate pirate;

use pirate::{Matches, Match, matches, usage, vars};

#[test]
fn main() {
    let env_args: Vec<String> = vec![String::from("test"), String::from("-a"), String::from("2"),
                                     String::from("3")];
    let opts = vec!["a/addend#The right side of the addition equation; default=1:", "#Required Arguments",
                    ":augend#The left side of an addition equation"];
    let mut vars = vars("test", &opts).unwrap();
    
    let matches: Matches = match matches(&env_args, &mut vars) {
        Ok(m) => m,
        Err(why) => {
            println!("Error: {}", why);
            usage(&vars);
            return;
        }
    };
    
    if matches.has_match("help") {
        usage(&vars);
        return;
    }
    
    let augend: i32 = matches.get("augend").unwrap().parse::<i32>().unwrap();

    let addend: i32 = match matches.get("addend") {
        Some(a) => a.parse::<i32>().unwrap(),
        None => 1
    };
    
    let sum = augend + addend;
    
    assert_eq!(augend, 3);
    assert_eq!(addend, 2);
    assert_eq!(sum, 5);
}
