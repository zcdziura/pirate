Pirate [![Build Status](https://travis-ci.org/zcdziura/pirate.svg?branch=master)](https://travis-ci.org/zcdziura/pirate)
======

A command-line arrrrguments parser, written in Rust.

Synopsis
--------

Most programs that provide a command-line interface use a special-purpose library to make the
process easier, such as the GNU Project's `getopt` library. The Rust team provides their own
alternative to `getopt`: `getopts`, which should win an award for the Most Originally Named Project
Ever.

In all seriousness, `getopts` is a fantastic library that gives the developers all of the tools
necessary to create and interface with command-line arguments. However, with all that power comes
complexity. `getopts` -- while straight forward to use -- is verbose. The developer has to call
different functions repeatedly in order to add different command-line options to their programs. And
while the only victim here is the developer's wrists due to carpal tunnel, I felt that there was a
better way to do things.

Enter Pirate (which should totally usurp `getopts` for the award of Most Originally Named Project Ever).

Installation
------------

Add this to your `Cargo.toml:

```
[dependencies]
pirate = "0.2.0"
```

and this to your crate root:

```rust
extern crate pirate;
```

Usage
-----

Using Pirate is simple. First, create a vector defining all of the valid opts that your
program accepts:

`let opts = vec!["o:", "l/long", ":arg"];`

Opts are defined in a specific format:

  * Opts that have an associated argument must be followed by a colon (:).
  * Opts with both a short and long form are separated by a slash (/). If an opt has an associated
    argument, the colon must come after the long form, e.g. `"l/long:"`.
  * Required program arguments have a preceding colon, e.g. `":arg"`.
  * All other opts are defined normally, e.g. `"l"` is an opt in short form, `"long"` is an opt in
    long form.

Next, call the `pirate::parse()` function, passing in the environment arguments along with a reference
to the opts that you defined:

`let matches = pirate::parse(env::args(), &opts);`

Now, handle any errors that may have arisen from parsing:

```
let matches: Matches = match pirate::parse(env::args(), &opts) {
    Err(ref e) => {
        println!("Error: {}", e);
        help();
        return;
    },
    Ok(m) => m
};
```

Finally, you may want to check which arguments were passed to the program. Luckily, the `Matches`
struct provides several helpful methods for querying whether an argument was passed to the program
and what its value is.

```
fn get(arg: &str) -> Option<&String> // Returns a reference to the given arg, or None if not found

fn has_arg(arg: &str) -> bool        // Returns true if the arg exists, false if not

fn keys() -> Keys<String, String>    // An iterator over all args passed to the program
```

Example
-------

Here is a trivial example that gives a general idea about how to use `pirate`:

```rust
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
```

To Do
-----

- [ ] Create a helper function for generating `--help` output, rather than having the user create it
manually.
  - [ ] Also create helper functions for defining the description section of the `--help` output.
- [x] Refactor the `ErrorKind` enum into a struct that is able to represent more complex data (such
  giving the value of the invalid argument passed to the program).

License
-------

Pirate is licensed under the [GNU Lesser General Public License, v3](https://www.gnu.org/licenses/lgpl.html).
