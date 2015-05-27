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

use lexer;

pub struct Matches {
    matches: HashMap<String, String>,
    pub tokens: Vec<lexer::Token>
}

impl Matches {
    pub fn new() -> Matches {
        Matches {
            matches: HashMap::new(),
            tokens: Vec::new()
        }
    }

    pub fn insert(&mut self, arg: &str, value: &str) {
        self.matches.insert(String::from(arg), String::from(value));
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
