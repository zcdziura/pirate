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

use std::collections::{HashMap, VecDeque};
use std::slice::Iter;

use errors::Error;
use token::{Token, token};

pub struct Vars {
    opts: HashMap<String, Token>,
    args: VecDeque<Token>,
    pub tokens: Vec<Token>,
    pub program_name: String
}

pub fn vars(program_name: &str, options: &[&str]) -> Result<Vars, Error> {
    let mut opts: HashMap<String, Token> = HashMap::new();
    let mut args: VecDeque<Token> = VecDeque::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut longest_token_len: usize = 0;

    for opt in options.iter() {
        let token = match token(opt) {
            Ok(t) => t,
            Err(why) => return Err(why)
        };

        if !token.is_group {
            if token.is_arg {
                args.push_back(token.clone());
            } else {
                if !token.short_name.is_empty() {
                    opts.insert(token.short_name.clone(), token.clone());
                }

                if !token.long_name.is_empty() {
                    opts.insert(token.long_name.clone(), token.clone());
                }
            }

            let token_len = token.len();
            if token_len > 0 {
                if token_len > longest_token_len {
                    longest_token_len = token_len;
                    for t in tokens.iter_mut() {
                        let diff = longest_token_len - t.len();
                        t.adjust_padding(diff);
                    }
                }
            }
        }
        tokens.push(token);
    }
    
    let help_token = Token {
        short_name: String::from("h"),
        long_name: String::from("help"),
        description: String::from("Display usage information"),
        is_arg: false,
        has_arg: false,
        is_group: false,
        padding: 0
    };

    opts.insert(String::from("-h"), help_token.clone());
    opts.insert(String::from("--help"), help_token.clone());

    Ok(Vars {
        opts: opts,
        args: args,
        tokens: tokens,
        program_name: String::from(program_name)
    })
}

impl Vars {
    pub fn get_opt(&self, opt_name: &String) -> Option<&Token> {
        self.opts.get(opt_name)
    }

    pub fn contains_opt(&self, opt: &String) -> bool {
        self.opts.contains_key(opt)
    }

    pub fn get_arg(&mut self) -> Option<Token> {
        self.args.pop_front()
    }

    pub fn arg_len(&self) -> usize {
        self.args.len()
    }
    
    pub fn tokens(&self) -> Iter<Token>{
        self.tokens.iter()
    }
}
