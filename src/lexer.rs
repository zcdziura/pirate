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

use errors::{Error, ErrorKind};

pub fn collect(input: &[&str]) -> Result<Vec<Token>, Error> {
    let mut vector: Vec<Token> = Vec::new();
    for item in input.iter() {
        match analyze(item) {
            Err(why) => return Err(why),
            Ok(item) => vector.push(item)
        }
    }
    
    Ok(vector)
}

pub fn analyze(input: &str) -> Result<Token, Error> {
    let mut token = Token::new();
    
    token.is_arg = match &input[..1] {
        ":" => true,
        _ => false
    };
    
    token.has_arg = match &input[(input.len() - 1)..] {
        ":" => true,
        _ => false
    };
    
    if token.is_arg && token.has_arg {
        return Err(Error::new(ErrorKind::OptionFormat, String::from(input)));
    }
    
    let option = if token.is_arg {
        &input[1..]
    } else if token.has_arg {
        &input[..(input.len() - 1)]
    } else {
        input
    };

    let mut current_stage = AnalysisStage::ShortName;
    for c in option.chars() {
        match c {
            '/' => current_stage = AnalysisStage::LongName,
            '(' => current_stage = AnalysisStage::Description,
            ')' => (),
            _ => {
                match current_stage {
                    AnalysisStage::ShortName => token.short_name.push(c),
                    AnalysisStage::LongName => token.long_name.push(c),
                    AnalysisStage::Description => token.description.push(c)
                }
            }
        }
    }
    
    if token.short_name.is_empty() && token.long_name.is_empty() {
        token.is_group = true;
    }

    Ok(token)
}

enum AnalysisStage {
    ShortName,
    LongName,
    Description
}

#[derive(Clone)]
pub struct Token {
    short_name: String,
    long_name: String,
    pub is_arg: bool,
    pub has_arg: bool,
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
    
    pub fn name(&self) -> &str {
        if !self.short_name.is_empty() {
            &self.short_name
        } else if !self.long_name.is_empty() {
            &self.long_name
        } else {
            ""   
        }
    }

    pub fn fmt_with_padding(&self, padding: usize) -> String {
        let mut name = format!("-{}, --{}", self.short_name, self.long_name);

        for _ in 0..padding {
            name.push(' ');
        }

        name
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let repr = format!("-{}, --{}    {}", self.short_name, self.long_name, self.description);
        write!(f, "{}", repr)
    }
}
