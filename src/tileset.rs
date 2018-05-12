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

use game::Position;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use shadow::ShadowFlags;
use std::cmp;

/// Represents a kind of tile.
#[derive(Copy, Clone, Debug)]
pub enum Tile {
    /// Standard floor tile
    Floor,
    /// Wall tile
    Wall,
    /// Rock tile
    Rock,
    /// Target square tile
    Square,
    /// Player tile
    Player,
    /// Shadow tile
    Shadow(ShadowFlags),
}

pub struct Tileset<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
    effective_height: u32,
    offset: i32,
}

impl<'a> Tileset<'a> {
    pub fn new(
        texture: Texture<'a>,
        width: u32,
        height: u32,
        effective_height: u32,
        offset: i32,
    ) -> Tileset<'a> {
        Tileset {
            texture: texture,
            width: width,
            height: height,
            effective_height: effective_height,
            offset: offset,
        }
    }

    /// Returns the associated texture
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Returns the width of a tile.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of a tile.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the effective height of a tile (used for stacking)
    pub fn effective_height(&self) -> u32 {
        self.effective_height
    }

    /// Returns the offset need to draw items on the floor.
    pub fn offset(&self) -> i32 {
        self.offset
    }

    /// Returns the location of the tile in the tileset texture.
    pub fn location(&self, tile: Tile) -> Option<(u32, u32)> {
        match tile {
            Tile::Floor => Some((0, 0)),
            Tile::Wall => Some((0, 2)),
            Tile::Rock => Some((2, 0)),
            Tile::Square => Some((1, 0)),
            Tile::Player => Some((3, 0)),
            Tile::Shadow(ShadowFlags::N_EDGE) => Some((4, 0)),
            Tile::Shadow(ShadowFlags::S_EDGE) => Some((5, 0)),
            Tile::Shadow(ShadowFlags::E_EDGE) => Some((0, 1)),
            Tile::Shadow(ShadowFlags::W_EDGE) => Some((1, 1)),
            Tile::Shadow(ShadowFlags::NE_CORNER) => Some((2, 1)),
            Tile::Shadow(ShadowFlags::NW_CORNER) => Some((3, 1)),
            Tile::Shadow(ShadowFlags::SE_CORNER) => Some((4, 1)),
            Tile::Shadow(ShadowFlags::SW_CORNER) => Some((5, 1)),
            Tile::Shadow(ShadowFlags { .. }) => None,
        }
    }

    /// Returns the top-left corner coordinates of the tile corresponding
    /// to the given position.
    pub fn get_coordinates(&self, pos: &Position) -> (i32, i32) {
        let x = self.width as i32 * pos.column();
        let y = self.effective_height as i32 * pos.row();
        (x, y)
    }

    /// Returns the full size needed to draw a level of the given dimensions.
    pub fn get_rendering_size(&self, extents: (i32, i32)) -> (u32, u32) {
        let width = extents.0 as u32 * self.width;
        let height = if extents.1 > 0 {
            self.height + (extents.1 - 1) as u32 * self.effective_height
        } else {
            0
        };
        (width, height)
    }

    /// Returns the Rect of the tile located at the given row and column in the texture.
    pub fn get_tile_rect(&self, col: u32, row: u32) -> Option<Rect> {
        let (w, h) = (self.width, self.height);
        let x = (col * w) as i32;
        let y = (row * h) as i32;
        Some(Rect::new(x, y, w, h))
    }
}

/// Enables selecting between two tilesets.
pub struct TilesetSelector<'a> {
    /// The extents of the current level
    extents: (i32, i32),
    /// The big tileset
    big_set: Tileset<'a>,
    /// The small tileset
    small_set: Tileset<'a>,
}

impl<'a> TilesetSelector<'a> {
    const THRESHOLD: i32 = 40;

    /// Creates a new instance.
    pub fn new(big_set: Tileset<'a>, small_set: Tileset<'a>) -> Self {
        TilesetSelector {
            extents: (0, 0),
            big_set: big_set,
            small_set: small_set,
        }
    }

    /// Resets the selector with the given extents.
    pub fn reset(&mut self, extents: (i32, i32)) {
        self.extents = extents;
    }

    pub fn select(&self) -> &Tileset {
        if cmp::max(self.extents.0, self.extents.1) > TilesetSelector::THRESHOLD {
            &self.small_set
        } else {
            &self.big_set
        }
    }
}
