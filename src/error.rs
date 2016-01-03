// This file is part of sokoban-rs
// Copyright 2015 Sébastien Watteau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::From;
use std::error;
use std::io;
use std::fmt::{self, Display, Formatter};
use super::game;

/// Represents an application error
#[derive(Debug)]
pub enum SokobanError {
    IoError(io::Error),
    ParseError(game::InvalidChar),
}

impl error::Error for SokobanError {
    fn description(&self) -> &str {
        match *self {
            SokobanError::IoError(..) => "I/O error",
            SokobanError::ParseError(..) => "Level parsing error",
        }
    }
}

impl Display for SokobanError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SokobanError::IoError(ref err) => write!(f, "{}", *err),
            SokobanError::ParseError(ref err) => write!(f, "{}", *err),
        }
    }
}

impl From<io::Error> for SokobanError {
    fn from(err: io::Error) -> Self {
        SokobanError::IoError(err)
    }
}

impl From<game::InvalidChar> for SokobanError {
    fn from(err: game::InvalidChar) -> Self {
        SokobanError::ParseError(err)
    }
}
