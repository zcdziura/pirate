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

use lexer;

pub struct Opts {
    pub opts: HashMap<String, bool>,
    pub args: VecDeque<String>,
}

impl Opts {
    pub fn new(options: &[&'static str]) -> Opts {
        let opts: Hashmap<String, bool> = HashMap::new();
        let args: VecDeque<String> = VecDeque::new();
        
        for opt in opts.iter() {
            if opt.is_arg {
                args.push_back(String::from(opt.name()));
            } else {
                opts.insert(String::from(opt.name()), opt.has_arg);
           }
        }
        
        opts.insert(String::from("-h", false));
        opts.insert(String::from("--help", false));
        
        Opts {
            opts: opts,
            args: args
        }
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