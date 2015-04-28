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

use std::error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    offender: String,
    desc: String,
}

impl Error {
    pub fn new(kind: ErrorKind, offender: String) -> Error {
        Error {
            kind: kind,
            offender: offender,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &format!("{} {}", self.kind.description(), self.offender);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.description())
    }
}

pub enum ErrorKind {
    InvalidOption,
    MissingArgument,
}

impl ErrorKind {
    fn description(&self) -> &str {
        match *self {
            ErrorKind::InvalidOption => "An invalid option was passed to the program:",
            ErrorKind::MissingArgument => "A required argument is missing:",
        }
    }
}

