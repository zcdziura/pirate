/* Pirate - A command-line arrrrguments parser, written in Rust.
 * Copyright (C) 2015 Zachary Dziura
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub mod errors;
pub mod matches;
mod opts;
mod token;

use std::env::Args;

pub use errors::{Error, ErrorKind};
pub use matches::Matches;
pub use lexer::{analyze, collect, Token};
use opts::{opts, Opts};

pub fn parse(mut args: Args, options: &[&'static str]) -> Result<Matches, Error> {
    let mut matches: Matches = Matches::new();

    let tokens = match lexer::collect(options) {
        Err(why) => return Err(why),
        Ok(t) => t
    };
    matches.tokens = tokens.clone();
    let mut opts: Opts = opts(&tokens);

    args.next(); // Remove the program name from the list of program arguments
    
    let mut next_arg = args.next();
    while next_arg.is_some() {
        let mut current_arg = next_arg.unwrap();
        let mut arg_vec: Vec<String> = Vec::new();

        // Determine if current opt is in short, long, or arg form
        if &current_arg[..1] == "-" {
            if &current_arg[..2] == "--" { // Long form opt
                arg_vec.push(String::from(&current_arg[2..]));
            } else { // Short form opt
                // Assuming it's a group of short-form opts; e.g. tar -xzf
                for c in current_arg[1..].chars() {
                    let mut s = String::new();
                    s.push(c);
                    arg_vec.push(s);
                }
            }

            for arg in arg_vec.iter() {
                if opts.contains_opt(&arg) {
                    let has_arg: bool = *opts.get_opt(&arg).unwrap();

                    if has_arg {
                        // NOTE: The corresponding arg MUST be immediately following
                        current_arg = match args.next() {
                            None => {
                                let arg_ = (*arg).clone();
                                return Err(Error::new(ErrorKind::MissingArgument, arg_));
                            },
                            Some(a) => a
                        };

                        matches.insert(&arg, &current_arg);
                    } else {
                        matches.insert(&arg, "");
                    }
                } else {
                    let arg_ = (*arg).clone();
                    return Err(Error::new(ErrorKind::InvalidArgument, arg_));
                }
            }
        } else { // Probably a required arg
            let arg_name: String = opts.get_arg().unwrap();
            matches.insert(&arg_name, &current_arg);
        }

        next_arg = args.next();
    }

    match opts.arg_len() {
        0 => Ok(matches),
        _ => Err(Error::new(ErrorKind::MissingArgument, opts.get_arg().unwrap())),
    }
}

pub fn help(tokens: &Vec<lexer::Token>, program_name: &str, program_desc: &str) {
    println!("{} - {}", program_name, program_desc);

    for token in tokens.iter() {
        println!("{}", token);
    }
}
