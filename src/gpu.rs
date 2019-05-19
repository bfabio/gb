use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;

use crate::memory::Memory;

const BYTES_PER_PIXEL: u8 = 4; // RGBA8888
const BUFFER_HEIGHT: u16 = 256;
const BUFFER_WIDTH: u16 = 256;

const BUFFER_SIZE: usize =
    (BUFFER_HEIGHT as usize * BUFFER_WIDTH as usize * BYTES_PER_PIXEL as usize);

pub struct GpuBuffer {
    pub buffer: [u8; BUFFER_SIZE],
}

impl GpuBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; BUFFER_SIZE],
        }
    }
}

pub struct Gpu {
    memory: Rc<RefCell<Memory>>,
}

impl Gpu {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Gpu {
        Gpu { memory }
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        texture: &mut Texture,
        buffer: &mut GpuBuffer,
    ) {
        let memory = self.memory.borrow();

        let mut tile_x: u8;
        let mut tile_y: u8;

        // BG Map Data 1
        for (i, tile_addr) in (0x9800..=0x9bff).enumerate() {
            let tile_num = memory.load(tile_addr);

            tile_x = (i % 32) as u8;
            tile_y = (i / 32) as u8;

            self.print_tile(self.get_tile(tile_num), &mut buffer.buffer, tile_x, tile_y);
        }
        texture
            .update(
                None,
                &buffer.buffer,
                BUFFER_WIDTH as usize * BYTES_PER_PIXEL as usize,
            )
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
    }

    fn get_tile(&self, tile_num: u8) -> [u8; 16] {
        let memory = self.memory.borrow();

        let tile_start = 0x8000 + u16::from(tile_num) * 16;
        let tile_end = tile_start + 16;

        let mut tile: [u8; 16] = [0; 16];
        let mut i: usize = 0;

        // Tile RAM
        for a in tile_start..tile_end {
            tile[i] = memory.load(a as usize);
            i += 1;
        }

        tile
    }

    fn print_tile(&self, tile: [u8; 16], buffer: &mut [u8], x: u8, y: u8) {
        assert!(x < 32);
        assert!(y < 32);

        let mut xx;
        let mut yy;
        for row in 0..=7 {
            let b = (tile[row * 2], tile[1 + row * 2]);

            for col in (0..=7).rev() {
                let color = if (b.0 & (1 << col)).count_ones() == 0 {
                    (0xff, 0xff, 0xff, 0xff)
                } else {
                    (0xff, 0x00, 0x00, 0x00)
                };

                xx = x as i32 * 8 + (col as i8 - 7).abs() as i32;
                yy = (y as i32 * 8) + row as i32;

                let index =
                    (xx as usize + yy as usize * BUFFER_WIDTH as usize) * BYTES_PER_PIXEL as usize;

                // 4 bytes per pixel
                buffer[index] = color.0;
                buffer[index + 1] = color.1;
                buffer[index + 2] = color.2;
                buffer[index + 3] = color.3;
            }
        }
    }
}

impl<'a> fmt::Display for Gpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "abc")
    }
}
