pub use minifb::{Key, Window, WindowOptions};
pub use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
    thread,
    time::Duration,
};

extern crate image;
pub use image::GenericImageView;

pub mod script {
    pub mod error;
    pub mod rand;
    pub mod r#trait;
}
pub mod r#struct {
    pub mod buffer;
    pub mod gui;
}
pub mod flow {
    pub mod render;
    pub mod event;
}

#[cfg(test)]
pub mod test;

#[cfg(test)]
pub use crate::test::*;

pub use crate::{
    r#struct::{
        buffer::*,
        gui::{button::*, gui::*, table::*, window::*},
    },
    script::{error::*, r#trait::*, rand::*},
    flow::{render::*, event::*},
};

fn main() {
    let size_window = [800, 600];

    let mut window = Window::new(
        "Game",
        size_window[0],
        size_window[1],
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer = BufferS::new(&size_window);

    {
        let image = ImageDataS::image_parsing("image.png", 5);
        image.set_photo_to_buffer([100, 200], &mut buffer);

        BufferS::draw_border(&[[500, 400], [200, 100]], 10, 0xFF_FF_00_00, &mut buffer);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*buffer = vec![0; size_window[0].mul(size_window[1])];

        draw_line(&mut buffer, size_window, p, 10);

        let i = 10;

        if window.is_key_down(Key::W) {
            p[0][1] -= i;
            p[1][1] -= i;
        } if window.is_key_down(Key::S) {
            p[0][1] += i;
            p[1][1] += i;
        } if window.is_key_down(Key::A) {
            p[0][0] -= i;
            p[1][0] -= i;
        } if window.is_key_down(Key::D) {
            p[0][0] += i;
            p[1][0] += i;
        }*/

        window
            .update_with_buffer(&buffer.buffer, size_window[0], size_window[1])
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        thread::sleep(Duration::from_secs_f64(0.05));
    }
}

//

#[derive(Debug, Clone)]
pub struct ImageDataS {
    image: Vec<u32>,
    size: [usize; 2],
}

impl ImageDataS {
    fn set_photo_to_buffer(self, position: [usize; 2], buffer: &mut BufferS) {
        let y = position[1] * buffer.size[0];
        let x = position[0];

        for i in 0..self.size[1] {
            let ny = y + i * buffer.size[0];

            for j in 0..self.size[0] {
                let nx = x + j;

                let index = ny + nx;
                buffer.buffer[index] = self.image[i * self.size[0] + j];
            }
        }

        //buffer.buffer[index_y + index_x] = 0xFF_00_00_FF;
    }

    fn image_parsing(path: &str, new_size: usize) -> Self {
        let image = image::open(path).expect("Failed to open image");

        let new_width = image.width() * new_size as u32;
        let new_height = image.height() * new_size as u32;
        let image = image.resize(new_width, new_height, image::imageops::FilterType::Nearest);

        let (width, height) = image.dimensions();

        let mut vector: Vec<u32> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let pix = image.get_pixel(x, y);

                let pix = [pix[0] as u32, pix[1] as u32, pix[2] as u32, pix[3] as u32];

                let color = (pix[3] << 24) | (pix[0] << 16) | (pix[1] << 8) | pix[2];

                vector.push(color as u32);
            }
        }

        Self {
            image: vector.clone(),
            size: [width as usize, height as usize],
        }
    }
}

/*
fn draw_line(
    buffer: &mut Vec<u32>,
    size_window: [usize; 2],
    posit_0: [usize; 2],
    posit_1: [usize; 2],
    line_width: usize,
) {
    let dx = ((posit_1[0] as isize).sub(posit_0[0] as isize)).abs();
    let dy = ((posit_1[1] as isize).sub(posit_0[1] as isize)).abs();
    let sx = if posit_0[0] < posit_1[0] { 1 } else { -1 };
    let sy = if posit_0[1] < posit_1[1] { 1 } else { -1 };
    let mut err = dx.sub(dy);

    let mut x = posit_0[0] as isize;
    let mut y = posit_0[1] as isize;

    while x != posit_1[0] as isize || y != posit_1[1] as isize {
        // Рисуем точку с учетом ширины линии
        for i in 0..line_width {
            for j in 0..line_width {
                let nx = ((x.add(i as isize)).sub(line_width as isize)).div(2);
                let ny = ((y.add(j as isize)).sub(line_width as isize)).div(2);

                if nx >= 0
                    && nx < size_window[0] as isize
                    && ny >= 0
                    && ny < size_window[1] as isize
                {
                    let index = (ny.mul((size_window[0] as isize).add(nx))) as usize;
                    buffer[index] = 0xFF_00_00_FF; // Красный цвет
                }
            }
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err = err.sub(dy);
            x = x.add(sx);
        }
        if e2 < dx {
            err = err.add(dx);
            y = y.add(sy);
        }
    }
}
*/
