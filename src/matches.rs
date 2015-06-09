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

use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::env;

use errors::{Error, ErrorKind};
use vars::Vars;

pub struct Matches {
    matches: HashMap<String, String>,
    program_name: String
}

impl Matches {
    pub fn new(opts: &mut Vars) -> Result<Matches, Error> {
        let mut args = env::args();
        let mut matches: HashMap<String, String> = HashMap::new();
        let program_name = args.next().unwrap();
        
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
                                None =>  return Err(Error::new(ErrorKind::MissingArgument, (*arg).clone())),
                                Some(a) => a
                            };

                            matches.insert(arg.clone(), current_arg);
                        } else {
                            matches.insert(arg.clone(), String::new());
                        }
                    } else {
                        return Err(Error::new(ErrorKind::InvalidArgument, arg.clone()));
                    }
                }
            } else { // Probably a required arg
                let arg_name: String = opts.get_arg().unwrap().clone();
                matches.insert(arg_name, current_arg);
            }

            next_arg = args.next();
        }

        match opts.arg_len() {
            0 => Ok(Matches { matches: matches, program_name: program_name }),
            _ => Err(Error::new(ErrorKind::MissingArgument, opts.get_arg().unwrap())),
        }
    }

    pub fn get(&self, arg: &str) -> Option<&String> {
        self.matches.get(arg)
    }

    pub fn has_arg(&self, arg: &str) -> bool {
        let arg = String::from(arg);
        self.matches.contains_key(&arg)
    }

    pub fn args(&self) -> Keys<String, String> {
        self.matches.keys()
    }
}
