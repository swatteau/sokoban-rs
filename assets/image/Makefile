# This file is part of sokoban-rs
# Copyright 2015 Sébastien Watteau
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# The original tileset is from Danc's Miraculously Flexible Game Prototyping Tiles
# (see http://www.lostgarden.com/2007/05/dancs-miraculously-flexible-game.html)
PNG_ZIP_URL=http://lostgarden.com/PlanetCute%20PNG.zip
PNG_ZIP=PlanetCute\ PNG.zip
PNG_DIR=PlanetCute\ PNG
PNG_FILES=$(PNG_DIR)/Plain\ Block.png \
          $(PNG_DIR)/Brown\ Block.png \
          $(PNG_DIR)/Rock.png \
          $(PNG_DIR)/Character\ Boy.png \
          $(PNG_DIR)/Shadow\ North.png \
          $(PNG_DIR)/Shadow\ South.png \
          $(PNG_DIR)/Shadow\ East.png \
          $(PNG_DIR)/Shadow\ West.png \
          $(PNG_DIR)/Shadow\ North\ East.png \
          $(PNG_DIR)/Shadow\ North\ West.png \
          $(PNG_DIR)/Shadow\ South\ East.png \
          $(PNG_DIR)/Shadow\ South\ West.png \
          $(PNG_DIR)/Wall\ Block.png

.PHONY: clean all fetch-png

all: fetch-png tileset.png tileset-small.png

tileset.png: $(PNG_FILES)
	gm montage -geometry 101x171 -background none $(PNG_FILES) tileset.png

tileset-small.png: $(PNG_FILES)
	gm montage -geometry 50x85 -background none $(PNG_FILES) tileset-small.png

fetch-png:
	wget $(PNG_ZIP_URL)
	unzip $(PNG_ZIP)

clean:
	rm -rf tileset.png tileset-small.png $(PNG_ZIP) $(PNG_DIR)

