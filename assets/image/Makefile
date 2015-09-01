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

PNG_FILES=Plain\ Block.png \
          Brown\ Block.png \
          Rock.png \
          Character\ Boy.png \
          Shadow\ North.png \
          Shadow\ South.png \
          Shadow\ East.png \
          Shadow\ West.png \
          Shadow\ North\ East.png \
          Shadow\ North\ West.png \
          Shadow\ South\ East.png \
          Shadow\ South\ West.png \
          Wall\ Block.png

.PHONY: clean all

all: tileset.png tileset-small.png

tileset.png: $(PNG_FILES)
	gm montage -geometry 101x171 -background none $(PNG_FILES) tileset.png

tileset-small.png: $(PNG_FILES)
	gm montage -geometry 50x85 -background none $(PNG_FILES) tileset-small.png

clean:
	rm -f tileset.png tileset-small.png

