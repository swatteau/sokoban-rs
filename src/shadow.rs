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

bitflags!(
    /// Represents the different kind of shadows that can be cast
    /// onto a floor tile.
    pub struct ShadowFlags: i32 {
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
