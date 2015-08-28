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

//! This is an implementation of Sokoban in Rust.

#[macro_use]
extern crate bitflags;
extern crate sdl2;
extern crate sdl2_image;
extern crate xml;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use sdl2::keyboard::Keycode;
use sdl2_image::INIT_PNG;
use xml::reader::EventReader;
use xml::reader::events::*;

pub mod game;
pub mod render;

use game::Level;
use render::Drawer;

pub fn main() {
    let slc_file = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| {
            println!("Argument missing: pleasy specify the path to an SLC file.");
            std::process::exit(1);
        });

    let sdl_context = sdl2::init().unwrap_or_else(|err| {
        println!("Failed to initialize an SDL context: {}", err);
        std::process::exit(1);
    });

    let video_subsystem = sdl_context.video().unwrap_or_else(|err| {
        println!("Failed to initialize the video subsystem: {}", err);
        std::process::exit(1);
    });

    sdl2_image::init(INIT_PNG);

    let window = video_subsystem.window("sokoban-rs", 1024, 768)
        .position_centered()
        //.fullscreen()
        .opengl()
        .build()
        .unwrap_or_else(|err| {
            println!("Failed to create the window: {}", err);
            std::process::exit(1);
        });

    let renderer = window.renderer()
        .build()
        .unwrap_or_else(|err| {
            println!("Failed to get an SDL renderer for the main window: {}", err);
            std::process::exit(1);
        });

    let mut drawer = Drawer::new(renderer);

    let mut collection = load_slc_file(Path::new(&slc_file)).into_iter();
    let mut reference_level = collection.next().unwrap();
    let mut level = reference_level.clone();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut skip = false;
    while running {
        if level.is_completed() || skip {
            match collection.next() {
                Some(l) => {
                    level = l;
                    reference_level = level.clone();
                    skip = false;
                },
                None => {
                    break;
                }
            }
        }
        drawer.draw(&level);
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    level.step(game::Direction::Left);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    level.step(game::Direction::Right);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    level.step(game::Direction::Up);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    level.step(game::Direction::Down);
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    level = reference_level.clone();
                }
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    skip = true;
                }
                _ => {}
            }
        }
    }
    sdl2_image::quit();
}

/// Builds levels from a level collection file in the SLC format.
fn load_slc_file(path: &Path) -> Vec<Level> {
    let mut collection = Vec::new();
    let reader = BufReader::new(File::open(&path).unwrap());
    let mut parser = EventReader::new(reader);

    let mut level_str = String::new();
    let mut reading_level = false;
    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                reading_level = name.local_name == "L";
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "Level" {
                    collection.push(Level::from_str(&level_str).unwrap());
                    level_str.clear();
                }
            },
            XmlEvent::Characters(ref data) => {
                if reading_level {
                    level_str.push_str(data);
                    level_str.push('\n');
                }
            }
            _ => {}
        }
    }

    collection
}
