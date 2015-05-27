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

use std::fmt::{Display, Formatter, Result};

use errors::{Error, ErrorKind};

pub fn analyze(input: &str) -> Result<Token, Error> {
    let mut token = Token::new();
    
    token.is_arg = match &opt[..1] {
        ":" => true,
        _ => false
    };
    
    token.has_arg = match &opt[(opt.len() - 1)..] {
        ":" => true,
        _ => false
    };
    
    if token.is_arg && token.has_arg {
        return Error::new(ErrorKind::OptionFormat, String::from(input));
    }
    
    let option = &input[1..(input.len() - 1)];

    let mut current_stage = AnalysisStage::ShortName;
    let mut current_char = option.chars().next();
    while current_char.is_some() {
        match current_char {
            '/' => {
                current_stage = AnalysisStage::LongName;
                continue;
            },
            '(' => {
                current_stage = AnalysisStage::Description;
                continue;
            },
            ')' => break,
            _ => ()
        }
        
        match current_stage {
            ShortName => token.short_name.push(current_char),
            LongName => token.long_name.push(current_char),
            Description => token.description.push(current_char)
        }
        
        current_char = current_char.next();
    }
    
    Ok(token)
}

enum AnalysisStage {
    ShortName,
    LongName,
    Description
}

pub struct Token {
    short_name: String,
    long_name: String,
    is_arg: bool,
    has_arg: bool,
    is_group: bool,
    description: String
}

impl Token {
    pub fn new() -> Token {
        Token {
            short_name: String::new(),
            long_name: String::new(),
            is_arg: false,
            has_arg: false,
            is_group: false,
            description: String::new()
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let repr = format!("-{}, --{}    {}", self.short_name, self.long_name, self.description);
        write(self, "{}", repr)
    }
}