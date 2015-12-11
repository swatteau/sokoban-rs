// This file is part of sokoban-rs
// Copyright 2015 Sébastien Watteau

image:https://travis-ci.org/swatteau/sokoban-rs.svg["Build Status", link="https://travis-ci.org/swatteau/sokoban-rs"]
//image:http://meritbadge.herokuapp.com/sokoban-rs["crates.io", link="https://crates.io/crates/sokoban-rs"]
image:https://img.shields.io/badge/crates.io-v1.0.3-brightgreen.svg["crates.io", link="https://crates.io/crates/sokoban-rs"]
image:https://img.shields.io/badge/license-Apache%202-blue.svg["License", link="https://www.apache.org/licenses/LICENSE-2.0"]

This is an implementation of Sokoban in the https://www.rust-lang.org[Rust Programming Language].

.An example level:
image:assets/image/screenshot.png["Screenshot",width=480,link="assets/image/screenshot.png"]

== Build instructions

Before building sokoban-rs, you will need to install the developpement libraries for https://www.libsdl.org[SDL2], preferably with the package manager that comes
with your operating system.

.Example for Debian/Ubuntu:
----
sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
----

.Example for Mac OSX
----
brew install sdl2
brew install sdl2_image
brew install sdl2_ttf
----

You might also like to read the README for these projects:

* https://github.com/AngryLawyer/rust-sdl2
* https://github.com/xsleonard/rust-sdl2_image
* https://github.com/andelf/rust-sdl2_ttf

To build sokoban-rs, type the following commands:

----
git clone https://github.com/swatteau/sokoban-rs
cd sokoban-rs
cargo build --release
----

== How to play

This game is released without any level. You can download level collections from http://www.sourcecode.se/sokoban/levels in the SLC (XML) format. For a quick start, try this:

----
wget http://www.sourcecode.se/sokoban/download/microban.slc
./target/release/sokoban-rs /path/to/microban.slc
----

* Use the arrow keys to move the player.
* Type R to retry the current level.
* Type N to skip the current level.

== Credits

* http://www.lostgarden.com/2007/05/dancs-miraculously-flexible-game.html["PlanetCute"] art by Daniel Cook (Lostgarden.com)
* Ruji's Handwriting Font by Ruji C. (rujic.net)

== License

----
Copyright 2015 Sébastien Watteau

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
----
