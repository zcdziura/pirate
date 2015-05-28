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
pub mod lexer;
pub mod matches;
mod opts;

use std::env::Args;

pub use errors::{Error, ErrorKind};
pub use matches::Matches;
use opts::Opts;

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
        let arg: String;

        if &current_arg[..1] == "-" { // Probably a opt
            if current_arg.len() == 2 { // Short form opt
                arg = String::from(&current_arg[1..]);
            } else { // Assuming it's a long form opt
                //TODO: Handle cases where it may be a opt group
                arg = String::from(&current_arg[2..]);
            }

            if opts.contains_opt(&arg) {
                let has_arg: bool = *opts.get_opt(&arg).unwrap();

                if has_arg {
                    // NOTE: The corresponding arg MUST be immediately following
                    current_arg = match args.next() {
                        None => {
                            return Err(Error::new(ErrorKind::MissingArgument, arg));
                        },
                        Some(a) => a
                    };
                    
                    matches.insert(&arg, &current_arg);
                } else {
                    matches.insert(&arg, "");
                }
            } else {
                return Err(Error::new(ErrorKind::InvalidArgument, arg));
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

fn opts(opts: &Vec<lexer::Token>) -> Opts {
    let mut options = Opts::new();

    for opt in opts.iter() {
        if opt.is_arg {
            options.insert_arg(opt.name());
        } else {
            options.insert_opt(opt.name(), opt.has_arg);
        }
    }

    // Push the obligatory "-h/--help" options
    options.insert_opt("h", false);
    options.insert_opt("help", false);

    options
}
