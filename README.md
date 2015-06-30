Pirate [![Build Status](https://travis-ci.org/zcdziura/pirate.svg?branch=master)](https://travis-ci.org/zcdziura/pirate)
======

A command-line arrrrguments parser, written in Rust.

Synopsis
--------

Most programs that provide a command-line interface use a special-purpose library to make the process easier, such as the GNU Project's `getopt` library. The Rust team provides their own alternative, `getopts`, which deserves an award for the Most Originally Named Project Ever.

In all seriousness, `getopts` is a fantastic library that gives the developers all of the power necessary to create and interface with command-line arguments. However, with all that power comes complexity. `getopts` -- while straight forward to use -- is verbose. The developer has to call different functions repeatedly in order to add different command-line options to their programs. While the only victim here is the developer's wrists due to carpal tunnel, I felt that there was a better way to do things.

Enter Pirate (which should totally usurp `getopts` for the award of Most Originally Named Project Ever).

Installation
------------

Add this to your project's `Cargo.toml` file:

```
[dependencies]
pirate = "1.0.0"
```

and this to your crate root:

```rust
extern crate pirate;
```

Usage
-----

Using Pirate is simple. First, create a vector defining all of the valid options that your program accepts:

```rust
let options = vec![
    "a/addend#The right side of the addition equation; default=1:",
    "#Required Arguments",
    ":/augend#The left side of an addition equation"
];
```

Options are defined in a very specific format:

  * Options that have an associated argument must be followed by a colon (:). The colon must be the last character of the option (see above for example).
  * Long-form options are denoted by a preceding slash (/). Options are able to have short- and long-forms. Options which are only long-form still need a preceding slash, e.g. `"/addend"`.
  * Required program arguments must have a preceding colon as the first character of the opt, e.g. `":/augend"`.
  * Option descriptions are denoted by a proceding hash (#). Descriptions are optional and are used to display helpful information about the option when displaying a program's usage information (typically when the `--help` flag is passed). Options with **only** a description (i.e. no short- or long-form name) are called "Groups", and are used to group options together when displaying usage.

Next, create a `Vars` struct, which is responsible for keeping track of all of the options, along with the program's name, defined for the program:

```rust
let vars: Vars = match pirate::vars("program-name", &options) {
    Ok(v) => v,
    Err(why) => panic!("Error: {}", why)
}
```

Next, call the `pirate::matches()` function, passing in a vector of the program's environment arguments, along with a mutable reference to the `Vars` struct that you previously defined:

```rust
let matches: Matches = match pirate::matches(env::args().collect(),
        &mut vars) {
    Ok(m) => m,
    Err(why) => {
        println!("Error: {}", why);
        pirate::usage(&vars);
        return;
    }
}
```
`Matches` is nothing more than a type alias to a `HashMap<String, String>`. All of the custom methods that make the type easier to use are defined by the `Match` trait.

And finally, check which arguments were passed to the program.

```
// Returns a reference to the given arg, or None if not found
fn get(arg: &str) -> Option<&String>;

// Returns true if the match exists, false if not
fn has_match(arg: &str) -> bool;

// An iterator over all matches found
fn keys() -> Keys<String, String>;
```

Something to remember when using the `get()` function: by default, the `pirate::matches()` function stores the opt's long-form name as the key, by default, should the long-form exist; otherwise the short-form is used. So, should you define an opt with both a short- and long-form name, when querying for it, pass the long-form as the argument. For example:

```rust
let options = vec!["l/long#An example opt"];
let vars = pirate::vars("program-name", &options);
let matches = pirate::matches(&env::args().collect(),
    &mut vars).unwrap();

let short = matches.get("l").unwrap(); // Error! This won't work!
let long = matches.get("long").unwrap(); // Success!

// Usage: program-name -l
```

As shown in a previous example, should you ever want to display the program's usage data, simply call the `pirate::usage()` function, passing in a reverence to your `Vars` struct as an argument. E.g. `pirate::usage(&vars)`

Example
-------

Here is a trivial example that gives a general idea about how to use `pirate`:

```rust
extern crate pirate;

use pirate::{Matches, Match, matches, usage, vars};

fn main() {
    let env_args: Vec<String> = vec![
        String::from("test"),
        String::from("-a"), String::from("2"),
        String::from("3")
    ];
    let options = vec![
        "a/addend#The right side of the addition equation; default=1:",
        "#Required Arguments",
        ":/augend#The left side of an addition equation"
    ];
    let mut vars = vars("test", &options).unwrap();
    
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
    
    let augend: i32 = matches.get("augend")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

    let addend: i32 = match matches.get("addend") {
        Some(a) => a.parse::<i32>().unwrap(),
        None => 1
    };
    
    let sum = augend + addend;
    
    println!("{} + {} = {}", augend, addend, sum);
}
```

License
-------

Pirate is licensed under the [GNU Lesser General Public License, v3](https://www.gnu.org/licenses/lgpl.html).
