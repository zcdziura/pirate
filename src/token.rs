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

#[derive(Clone)]
pub struct Token {
    short_name: String,
    long_name: String,
    pub is_arg: bool,
    pub has_arg: bool,
    is_group: bool,
    description: String,
    padding: usize
}

impl Token {
    pub fn new(input: &str) -> Result<Token, Error> {
        let mut short_name = String::new();
        let mut long_name = String::new();
        let mut description = String::new();
        let last_char = input.len() - 1;
        
        let is_arg = match &input[..1] {
            ":" => true,
            _ => false
        };

        let has_arg = match &input[last_char..] {
            ":" => true,
            _ => false
        };

        if is_arg && has_arg {
            return Err(Error::new(ErrorKind::OptionFormat, String::from(input)));
        }

        let option = if is_arg {
            input[1..]
        } else if has_arg {
            input[..last_char]
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
                        AnalysisStage::ShortName => short_name.push(c),
                        AnalysisStage::LongName => long_name.push(c),
                        AnalysisStage::Description => description.push(c)
                    }
                }
            }
        }

        let is_group = if short_name.is_empty() && long_name.is_empty() {
            true
        } else {
            false
        };

        Ok(Token {
            short_name: short_name,
            long_name: long_name,
            is_arg: is_arg,
            has_arg: has_arg,
            is_group: is_group,
            description: description,
            padding: 0
        })
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
    
    pub fn adjust_padding(&self, padding: usize) {
        self.padding = padding;
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut spacing = String::new();
        for 0..self.padding {
            spacing.push(' ');
        }
        
        let repr = if self.is_group {
            format!("{}:", self.description)
        } else {
            format!("  -{}, --{}{}{}", self.short_name, self.long_name, spacing self.description)
        };

        write!(f, "{}", repr)
    }
}

enum AnalysisStage {
    ShortName,
    LongName,
    Description
}