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
use std::collections::VecDeque;

use errors::Error;
use token::Token;

pub struct Vars {
    pub opts: HashMap<String, bool>,
    pub args: VecDeque<String>,
    pub tokens: Vec<Token>
}

impl Vars {
    pub fn new(options: &[&str]) -> Result<Vars, Error> {
        let mut opts: HashMap<String, bool> = HashMap::new();
        let mut args: VecDeque<String> = VecDeque::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut longest_token_len: usize = 0;
        
        for opt in options.iter() {
            let token = match Token::new(opt) {
                Ok(t) => t,
                Err(why) => return Err(why)
            };
            
            if !token.is_group {
                if token.is_arg {
                    args.push_back(String::from(token.name()));
                } else {
                    opts.insert(String::from(token.name()), token.has_arg);
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
        
        opts.insert(String::from("-h"), false);
        opts.insert(String::from("--help"), false);
        
        Ok(Vars {
            opts: opts,
            args: args,
            tokens: tokens
        })
    }

    pub fn get_opt(&self, opt_name: &String) -> Option<&bool> {
        self.opts.get(opt_name)
    }

    pub fn contains_opt(&self, opt: &String) -> bool {
        self.opts.contains_key(opt)
    }

    pub fn get_arg(&mut self) -> Option<String> {
        self.args.pop_front()
    }

    pub fn arg_len(&self) -> usize {
        self.args.len()
    }
}
