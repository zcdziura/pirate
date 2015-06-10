extern crate pirate;

use pirate::{Matches, usage, Vars};

fn main() {
    let opts = vec!["n:", "b/boop", ":input"];
    let mut vars = match Vars::new(&opts) {
        Ok(v) => v,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    let matches = match Matches::new(&mut vars) {
        Ok(m) => m,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };

    if matches.has_arg("-h") || matches.has_arg("--help") {
        usage(&vars);
        return;
    }
}
