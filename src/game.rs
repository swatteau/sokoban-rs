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

use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Represents a direction.
#[derive(Copy, Clone)]
pub enum Direction {
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
}

/// Represents a position in the world.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(i32, i32);

impl Position {
    /// Creates a new position with the given column and row.
    pub fn new(row: i32, col: i32) -> Position {
        Position(row, col)
    }

    /// Returns the position that is next to this one in the given direction.
    pub fn neighbor(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position(self.0 - 1, self.1),
            Direction::Down => Position(self.0 + 1, self.1),
            Direction::Left => Position(self.0, self.1 - 1),
            Direction::Right => Position(self.0, self.1 + 1),
        }
    }

    /// Returns the row number.
    pub fn row(&self) -> i32 {
        self.0
    }

    /// Returns the column number.
    pub fn column(&self) -> i32 {
        self.1
    }
}

/// Represents the state of the level.
#[derive(Clone)]
pub struct Level {
    /// The level's title
    title: String,
    /// The player's position
    player: Position,
    /// The current number of steps
    steps: i32,
    /// The positions of the walls
    walls: HashSet<Position>,
    /// The positions of the boxes
    boxes: HashSet<Position>,
    /// The positions of the squares
    squares: HashSet<Position>,
    /// The number of columns and rows in the level
    extents: (i32, i32),
}

impl Level {
    /// Moves the player in the given direction if possible.
    pub fn step(&mut self, dir: Direction) {
        let next_to_player = self.player.neighbor(dir);
        if self.is_free(&next_to_player) {
            self.move_player(next_to_player);
        } else if self.is_box(&next_to_player) {
            let next_to_box = next_to_player.neighbor(dir);
            if self.is_free(&next_to_box) {
                self.move_box(&next_to_player, next_to_box);
                self.move_player(next_to_player);
            }
        }
    }

    /// Returns the current number of steps.
    pub fn get_steps(&self) -> i32 {
        self.steps
    }

    /// Returns true if the level is completed.
    pub fn is_completed(&self) -> bool {
        self.squares.difference(&self.boxes).count() == 0
    }

    /// Returns true if the given location is free.
    pub fn is_free(&self, pos: &Position) -> bool {
        !self.walls.contains(pos) && !self.boxes.contains(pos)
    }

    /// Returns true if there is a box at the given position.
    pub fn is_box(&self, pos: &Position) -> bool {
        self.boxes.contains(pos)
    }

    /// Returns true if the player is at the given position.
    pub fn is_player(&self, pos: &Position) -> bool {
        self.player == *pos
    }

    /// Returns true if there is a square at the given position.
    pub fn is_square(&self, pos: &Position) -> bool {
        self.squares.contains(pos)
    }

    /// returns true if there is a wall at the given position.
    pub fn is_wall(&self, pos: &Position) -> bool {
        self.walls.contains(pos)
    }

    /// Returns the number of columns and rows of this level.
    pub fn extents(&self) -> (i32, i32) {
        self.extents
    }

    /// Returns the title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Changes the title
    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.title = title.into();
    }

    /// moves the player to the given position.
    fn move_player(&mut self, pos: Position) {
        if pos != self.player {
            self.player = pos;
            self.steps += 1;
        }
    }

    /// moves a box from a position to another position.
    fn move_box(&mut self, from: &Position, to: Position) {
        if self.boxes.remove(from) {
            self.boxes.insert(to);
        }
    }
}

/// Represents an error due to reading an invalid character.
#[derive(Debug)]
pub struct InvalidChar(char, Position);

impl Display for InvalidChar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let InvalidChar(c, pos) = *self;
        write!(
            f,
            "invalid character `{}' at row {}, column {}",
            c,
            pos.row(),
            pos.column()
        )
    }
}

impl FromStr for Level {
    type Err = InvalidChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut level = Level {
            title: String::new(),
            player: Position(0, 0),
            steps: 0,
            walls: HashSet::new(),
            boxes: HashSet::new(),
            squares: HashSet::new(),
            extents: (0, 0),
        };

        let (mut row, mut col) = (0, 0);
        for c in s.chars() {
            let pos = Position(row, col);
            match c {
                '\n' => {
                    row += 1;
                    col = -1;
                }
                '#' => {
                    level.walls.insert(pos);
                }
                '.' => {
                    level.squares.insert(pos);
                }
                '$' => {
                    level.boxes.insert(pos);
                }
                '@' => {
                    level.player = pos;
                }
                '+' => {
                    level.player = pos;
                    level.squares.insert(pos);
                }
                '*' => {
                    level.boxes.insert(pos);
                    level.squares.insert(pos);
                }
                ' ' => {}
                _ => {
                    return Err(InvalidChar(c, pos));
                }
            }
            col += 1;
        }

        // Calculate the extents of the level
        let (mut w, mut h) = (level.player.column(), level.player.row());
        for pos in level
            .walls
            .iter()
            .chain(level.squares.iter())
            .chain(level.boxes.iter())
        {
            if pos.column() > w {
                w = pos.column();
            }
            if pos.row() > h {
                h = pos.row();
            }
        }
        level.extents = (w + 1, h + 1);

        Ok(level)
    }
}
