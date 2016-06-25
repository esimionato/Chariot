//
// OpenAOE: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use empires::EmpiresDb;
use error::*;

use io_tools::*;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct PlayerColor {
    name: String,
    id: u16,
    palette_index: u8,
}

impl PlayerColor {
    fn new() -> PlayerColor {
        Default::default()
    }
}

impl EmpiresDb {
    pub fn read_player_colors<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let color_count = try!(cursor.read_u16());
        for _ in 0..color_count {
            let mut color = PlayerColor::new();
            color.name = try!(cursor.read_sized_str(30));
            color.id = try!(cursor.read_u16());
            try!(cursor.read_u16()); // unknown; skip

            color.palette_index = try!(cursor.read_byte());
            try!(cursor.read_byte()); // unknown byte

            self.player_colors.push(color);
        }

        Ok(())
    }
}