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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    pub is_arg: bool,
    pub has_arg: bool,
    pub is_group: bool,
    pub padding: usize
}

pub fn token(input: &str) -> Result<Token, Error> {
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
        return Err(Error::new(ErrorKind::TokenFormat, String::from(input)));
    }

    let option = if is_arg {
        &input[1..]
    } else if has_arg {
        &input[..last_char]
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

impl Token {
    pub fn adjust_padding(&mut self, padding: usize) {
        self.padding = padding;
    }
    
    pub fn len(&self) -> usize {
        let short_name_empty = self.short_name.is_empty();
        let long_name_empty = self.long_name.is_empty();
        
        let repr = if !short_name_empty && !long_name_empty {
                format!("-{}, --{}", self.short_name, self.long_name)
            } else if !short_name_empty && long_name_empty {
                format!("-{}", self.short_name)
            } else if short_name_empty && !long_name_empty {
                format!("--{}", self.long_name)
            } else {
                String::new()
            };
        
        repr.len()
    }
    
    pub fn name(&self) -> String {
        if !self.long_name.is_empty() {
            self.long_name.clone()
        } else if !self.short_name.is_empty() {
            self.short_name.clone()
        } else {
            String::new()
        }
    }
    
    pub fn usage(&self) -> Option<String> {
        let mut repr = String::new();
        
        if !self.is_group {
            if !self.is_arg {
                repr.push('[');
                
                if !self.short_name.is_empty() {
                    repr.push('-');
                    repr.push_str(&self.short_name);
                }

                if !self.long_name.is_empty() {
                    repr.push_str(" | --");
                    repr.push_str(&self.long_name);
                }

                if self.has_arg {
                    let name = String::from(self.name());
                    repr.push(' ');
                    repr.push_str(&name);
                }
                
                repr.push(']');
            } else {
                let name = String::from(self.name());
                repr.push_str(&name);
            }
            
            Some(repr)
        } else {
            None
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut spacing = String::new();
        for _ in 0..self.padding {
            spacing.push(' ');
        }
        
        let repr = if self.is_group {
            format!("{}:", self.description)
        } else {
            format!("  -{}, --{}{}  {}", self.short_name, self.long_name, spacing, self.description)
        };

        write!(f, "{}", repr)
    }
}

enum AnalysisStage {
    ShortName,
    LongName,
    Description
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn test_new_token() {
        let opt = "h/help(Display the program usage)";
        let token = match Token::new(opt) {
            Ok(t) => t,
            Err(why) => panic!("Received error: {}", why)
        };
        let control_token = Token {
            short_name: String::from("h"),
            long_name: String::from("help"),
            description: String::from("Display the program usage"),
            is_arg: false,
            has_arg: false,
            is_group: false,
            padding: 0
        };

        assert_eq!(token, control_token);
    }

    #[test]
    fn test_new_group() {
        let opt = "(This is a group)";
        let token = match Token::new(opt) {
            Ok(t) => t,
            Err(why) => panic!("Received error: {}", why)
        };
        let control_token = Token {
            short_name: String::new(),
            long_name: String::new(),
            description: String::from("This is a group"),
            is_arg: false,
            has_arg: false,
            is_group: true,
            padding: 0
        };

        assert_eq!(token, control_token);
    }

    #[test]
    fn test_new_token_with_arg() {
        let opt = "o/option(An option with an argument):";
        let token = match Token::new(opt) {
            Ok(t) => t,
            Err(why) => panic!("Received error: {}", why)
        };
        let control_token = Token {
            short_name: String::from("o"),
            long_name: String::from("option"),
            description: String::from("An option with an argument"),
            is_arg: false,
            has_arg: true,
            is_group: false,
            padding: 0
        };

        assert_eq!(token, control_token);
    }

    #[test]
    fn test_new_token_as_arg() {
        let opt = ":a/arg(An argument)";
        let token = match Token::new(opt) {
            Ok(t) => t,
            Err(why) => panic!("Received error: {}", why)
        };
        let control_token = Token {
            short_name: String::from("a"),
            long_name: String::from("arg"),
            description: String::from("An argument"),
            is_arg: true,
            has_arg: false,
            is_group: false,
            padding: 0
        };

        assert_eq!(token, control_token);
    }

    #[test]
    #[should_panic]
    fn test_invalid_token_format() {
        let input = ":w/wrong(Wrong format):";
        match Token::new(input) {
            Ok(t) => t,
            Err(why) => panic!("Received error: {}", why)
        };
    }

    #[test]
    fn test_name() {
        let short_name = "o";
        let long_name = "o/out";
        let group = "(Output)";
        
        let short_token = Token::new(short_name).unwrap();
        let long_token = Token::new(long_name).unwrap();
        let group_token = Token::new(group).unwrap();
        
        assert_eq!(short_token.name(), "o");
        assert_eq!(long_token.name(), "out");
        assert_eq!(group_token.name(), "");
    }
}
