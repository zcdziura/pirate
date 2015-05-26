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

use std::fmt::{self, Display, Formatter};

pub fn analyze(input: &str) -> Result {
    
}

pub struct Result {
    short_name: String,
    long_name: String,
    is_arg: bool,
    has_arg: bool,
    description: String
}

impl Result {
    pub fn new(short_name: &str, long_name: &str, is_arg: bool, has_arg: bool, description: &str)
        -> Result {
        Result {
            short_name: String::from(short_name),
            long_name: String::from(long_name),
            is_arg: is_arg,
            has_arg: has_arg,
            description: String::from(description)
        }
    }
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let repr = format!("-{}, --{}    {}", self.short_name, self.long_name, self.description);
        write(self, "{}", repr)
    }
}