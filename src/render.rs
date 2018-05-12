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

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use sdl2_ttf::{Font, Sdl2TtfContext};
use std::cmp;
use std::ops::Deref;
use std::path::Path;

use game::{Direction, Level, Position};

/// The Drawer struct is responsible for drawing the game onto the screen.
pub struct Drawer<'a> {
    /// The underlying SDL renderer
    renderer: Renderer<'a>,
    /// The active tileset
    tileset: TileSetSwitch,
    /// The font used to display text
    font: Font<'a>,
    /// The size of the screen in pixels
    screen_size: (u32, u32),
    /// The height of the status bar
    bar_height: u32,
    /// The color of the status bar
    bar_color: Color,
    /// The color of the text in the status bar
    bar_text_color: Color,
}

/// Represents a location for text in the status bar
enum StatusBarLocation {
    FlushLeft,
    FlushRight,
}

impl<'a> Drawer<'a> {
    /// Creates a new Drawer instance.
    pub fn new(renderer: Renderer<'a>, ttf_context: &'a Sdl2TtfContext) -> Drawer<'a> {
        let font = {
            let ttf = Path::new("assets/font/RujisHandwritingFontv.2.0.ttf");
            ttf_context.load_font(&ttf, 20).unwrap()
        };
        let screen_size = renderer.window().unwrap().drawable_size();
        let tileset = TileSetSwitch::new(&renderer);
        Drawer {
            renderer: renderer,
            tileset: tileset,
            font: font,
            screen_size: screen_size,
            bar_height: 32,
            bar_color: Color::RGBA(20, 20, 20, 255),
            bar_text_color: Color::RGBA(255, 192, 0, 255),
        }
    }

    /// Draws a level onto the screen.
    pub fn draw(&mut self, level: &Level) {
        self.tileset.set_extents(level.extents());

        // Draw a full-size image onto an off-screen buffer
        let fullsize = self.tileset.get_rendering_size(level.extents());
        let _ = self.renderer
            .render_target()
            .expect("Render targets are not supported")
            .create_and_set(PixelFormatEnum::RGBA8888, fullsize.0, fullsize.1);

        self.draw_fullsize(level);

        // Copy onto the screen with appropriate scaling
        let final_rect = self.get_centered_image_rect(self.get_scaled_rendering_size(&level));
        let texture = self.renderer
            .render_target()
            .unwrap()
            .reset()
            .unwrap_or_else(|err| panic!("Could not reset to the default render target: {}", err))
            .unwrap_or_else(|| panic!("Could not get the offscreen texture"));

        self.renderer.clear();
        let original_rect = Some(Rect::new(0, 0, fullsize.0, fullsize.1));
        self.renderer
            .copy(&texture, original_rect, final_rect)
            .unwrap();

        self.draw_status_bar(&level);

        self.renderer.present();
    }

    /// Draws a full-size image of the given level onto the current render target.
    fn draw_fullsize(&mut self, level: &Level) {
        let (cols, rows) = level.extents();
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        for r in 0..rows {
            for c in 0..cols {
                let pos = Position::new(r, c);
                let (x, y) = self.tileset.get_coordinates(&pos);

                // First draw the floor tiles
                if level.is_square(&pos) {
                    self.draw_tile(Tile::Square, x, y);
                } else {
                    self.draw_tile(Tile::Floor, x, y);
                }

                // Add the shadows
                let flags = get_shadow_flags(&level, &pos);
                for f in &[
                    ShadowFlags::N_EDGE,
                    ShadowFlags::S_EDGE,
                    ShadowFlags::E_EDGE,
                    ShadowFlags::W_EDGE,
                    ShadowFlags::NE_CORNER,
                    ShadowFlags::NW_CORNER,
                    ShadowFlags::SE_CORNER,
                    ShadowFlags::SW_CORNER,
                ] {
                    if flags.contains(*f) {
                        self.draw_tile(Tile::Shadow(*f), x, y);
                    }
                }

                // Draw the other items
                let z = y - self.tileset.tile_offset();
                if level.is_wall(&pos) {
                    self.draw_tile(Tile::Wall, x, z);
                }
                if level.is_box(&pos) {
                    self.draw_tile(Tile::Rock, x, z);
                }
                if level.is_player(&pos) {
                    self.draw_tile(Tile::Player, x, z);
                }
            }
        }
    }

    /// Draws the status bar
    fn draw_status_bar(&mut self, level: &Level) {
        let prev_color = self.renderer.draw_color();
        self.renderer.set_draw_color(self.bar_color);
        let rect = Rect::new(
            0,
            (self.screen_size.1 - self.bar_height) as i32,
            self.screen_size.0,
            self.bar_height,
        );
        self.renderer.fill_rect(rect).unwrap();
        self.renderer.set_draw_color(prev_color);

        // Draw the number of moves
        let s = format!("# moves: {}", level.get_steps());
        self.draw_status_text(&s, StatusBarLocation::FlushLeft);

        // Draw the level's title
        self.draw_status_text(level.title(), StatusBarLocation::FlushRight);
    }

    /// Draws text in the status bar
    fn draw_status_text(&mut self, text: &str, location: StatusBarLocation) {
        let surface = self.font.render(text).blended(self.bar_text_color).unwrap();
        let texture = self.renderer.create_texture_from_surface(&surface).unwrap();
        let margin = 4;
        let (w, h) = {
            let q = texture.query();
            (q.width, q.height)
        };
        let (x, y) = match location {
            StatusBarLocation::FlushLeft => {
                (margin as i32, (self.screen_size.1 - margin - h) as i32)
            }
            StatusBarLocation::FlushRight => (
                (self.screen_size.0 - margin - w) as i32,
                (self.screen_size.1 - margin - h) as i32,
            ),
        };
        self.renderer
            .copy(&texture, None, Some(Rect::new(x, y, w, h)))
            .unwrap();
    }

    /// Draws a tile at the given coordinates.
    fn draw_tile(&mut self, tile: Tile, x: i32, y: i32) {
        let (col, row) = self.tileset.location(tile).unwrap_or_else(|| {
            panic!("No image for this tile: {:?}", tile);
        });
        let tile_rect = self.tileset.get_tile_rect(col, row);
        let target_rect = Some(Rect::new(
            x,
            y,
            self.tileset.tile_width(),
            self.tileset.tile_height(),
        ));
        self.renderer
            .copy(self.tileset.texture(), tile_rect, target_rect)
            .unwrap();
    }

    /// Returns the size of the drawing scaled to fit onto the screen.
    fn get_scaled_rendering_size(&self, level: &Level) -> (u32, u32) {
        let render_size = self.tileset.get_rendering_size(level.extents());
        let width_ratio = (self.screen_size.0 as f64) / (render_size.0 as f64);
        let h = self.screen_size.1 - self.bar_height;
        let height_ratio = (h as f64) / (render_size.1 as f64);
        let ratio = f64::min(1.0, f64::min(width_ratio, height_ratio));

        let scale = |sz: u32| (ratio * (sz as f64)).floor() as u32;

        (scale(render_size.0), scale(render_size.1))
    }

    /// Returns the Rect of an image of given dimensions so that it's centered on the screen.
    fn get_centered_image_rect(&self, img_size: (u32, u32)) -> Option<Rect> {
        let x = (self.screen_size.0 - img_size.0) as i32 / 2;
        let y = (self.screen_size.1 - self.bar_height - img_size.1) as i32 / 2;
        Some(Rect::new(x, y, img_size.0, img_size.1))
    }
}

/// Represents a kind of tile.
#[derive(Copy, Clone, Debug)]
enum Tile {
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

/// Holds information about a tileset.
trait TileSet {
    /// Returns the associated texture
    fn texture(&self) -> &Texture;

    /// Returns the width of a tile.
    fn tile_width(&self) -> u32;

    /// Returns the height of a tile.
    fn tile_height(&self) -> u32;

    /// Returns the effective height of a tile (used for stacking)
    fn tile_effective_height(&self) -> u32;

    /// Returns the offset need to draw items on the floor.
    fn tile_offset(&self) -> i32;

    /// Returns the location of the tile in the tileset texture.
    fn location(&self, tile: Tile) -> Option<(u32, u32)> {
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
    fn get_coordinates(&self, pos: &Position) -> (i32, i32) {
        let x = self.tile_width() as i32 * pos.column();
        let y = self.tile_effective_height() as i32 * pos.row();
        (x, y)
    }

    /// Returns the full size needed to draw a level of the given dimensions.
    fn get_rendering_size(&self, extents: (i32, i32)) -> (u32, u32) {
        let width = extents.0 as u32 * self.tile_width();
        let height = if extents.1 > 0 {
            self.tile_height() + (extents.1 - 1) as u32 * self.tile_effective_height()
        } else {
            0
        };
        (width, height)
    }

    /// Returns the Rect of the tile located at the given row and column in the texture.
    fn get_tile_rect(&self, col: u32, row: u32) -> Option<Rect> {
        let (w, h) = (self.tile_width(), self.tile_height());
        let x = (col * w) as i32;
        let y = (row * h) as i32;
        Some(Rect::new(x, y, w, h))
    }
}

// A macro to generate the tilesets
macro_rules! decl_tileset {
    ($name:ident, $path:expr, $width:expr, $height:expr, $effective_height:expr, $offset:expr) => {
        struct $name {
            texture: Texture,
        }
        impl $name {
            pub fn new(renderer: &Renderer) -> Self {
                $name {
                    texture: renderer.load_texture(Path::new($path)).unwrap(),
                }
            }
        }
        impl TileSet for $name {
            fn texture(&self) -> &Texture {
                &self.texture
            }
            fn tile_width(&self) -> u32 {
                $width
            }
            fn tile_height(&self) -> u32 {
                $height
            }
            fn tile_effective_height(&self) -> u32 {
                $effective_height
            }
            fn tile_offset(&self) -> i32 {
                $offset
            }
        }
    };
}

decl_tileset!(BigTileSet, "assets/image/tileset.png", 101, 171, 83, 40);
decl_tileset!(
    SmallTileSet,
    "assets/image/tileset-small.png",
    50,
    85,
    41,
    20
);

/// Enables switching between two tilesets.
struct TileSetSwitch {
    /// The limit value for switching
    limit: i32,
    /// The extents of the current level
    extents: (i32, i32),
    /// The small tileset
    tileset_small: SmallTileSet,
    /// The big tileset
    tileset_big: BigTileSet,
}

impl TileSetSwitch {
    /// Creates a new instance.
    pub fn new(renderer: &Renderer) -> Self {
        TileSetSwitch {
            limit: 40,
            extents: (0, 0),
            tileset_big: BigTileSet::new(renderer),
            tileset_small: SmallTileSet::new(renderer),
        }
    }

    /// Updates the switch with the given extents.
    pub fn set_extents(&mut self, extents: (i32, i32)) {
        self.extents = extents;
    }
}

// Select the appropriate tileset while dereferencing
impl Deref for TileSetSwitch {
    type Target = TileSet;
    fn deref(&self) -> &Self::Target {
        if cmp::max(self.extents.0, self.extents.1) > self.limit {
            &self.tileset_small
        } else {
            &self.tileset_big
        }
    }
}

bitflags!(
    /// Represents the different kind of shadows that can be cast
    /// onto a floor tile.
    struct ShadowFlags: i32 {
        /// North edge
        const N_EDGE = 0x1;
        /// South edge
        const S_EDGE = 0x2;
        /// East edge
        const E_EDGE = 0x4;
        /// West edge
        const W_EDGE = 0x8;
        /// North East corner
        const NE_CORNER = 0x10;
        /// North West corner
        const NW_CORNER = 0x20;
        /// South East corner
        const SE_CORNER = 0x40;
        /// South West corner
        const SW_CORNER = 0x80;
    }
);

/// Returns the shadow flags for a particular position in the given level.
fn get_shadow_flags(level: &Level, pos: &Position) -> ShadowFlags {
    let north = pos.neighbor(Direction::Up);
    let south = pos.neighbor(Direction::Down);
    let west = pos.neighbor(Direction::Left);
    let east = pos.neighbor(Direction::Right);

    let mut flags = ShadowFlags::empty();
    if level.is_wall(&north) {
        flags = flags | ShadowFlags::N_EDGE;
    }
    if level.is_wall(&south) {
        flags = flags | ShadowFlags::S_EDGE;
    }
    if level.is_wall(&west) {
        flags = flags | ShadowFlags::W_EDGE;
    }
    if level.is_wall(&east) {
        flags = flags | ShadowFlags::E_EDGE;
    }
    if level.is_wall(&north.neighbor(Direction::Right))
        && !flags.intersects(ShadowFlags::N_EDGE | ShadowFlags::E_EDGE)
    {
        flags = flags | ShadowFlags::NE_CORNER;
    }
    if level.is_wall(&north.neighbor(Direction::Left))
        && !flags.intersects(ShadowFlags::N_EDGE | ShadowFlags::W_EDGE)
    {
        flags = flags | ShadowFlags::NW_CORNER;
    }
    if level.is_wall(&south.neighbor(Direction::Right))
        && !flags.intersects(ShadowFlags::S_EDGE | ShadowFlags::E_EDGE)
    {
        flags = flags | ShadowFlags::SE_CORNER;
    }
    if level.is_wall(&south.neighbor(Direction::Left))
        && !flags.intersects(ShadowFlags::S_EDGE | ShadowFlags::W_EDGE)
    {
        flags = flags | ShadowFlags::SW_CORNER;
    }
    flags
}
