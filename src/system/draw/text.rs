use crate::*;

impl FontS {
    #[cfg(feature = "draw")]
    pub fn draw(
        &self,
        position: [f64; 2],
        size: f64,
        text: String,
        display: &mut PistonWindow,
        event: &Event,
    ) {
        let mut position = position.clone();

        display.draw_2d(event, |context, graphics, _| {
            for r#char in text.chars() {
                let array_pixel = self.get_pixel_positions(r#char);

                for (y, array) in array_pixel.iter().enumerate() {
                    for (x, pixel) in array.iter().enumerate() {
                        if pixel == &1 || pixel == &2 || pixel == &3 {
                            let color = match pixel {
                                1 => [1.0, 0.0, 0.0, 1.0],
                                2 => [0.0, 1.0, 0.0, 1.0],
                                3 => [0.0, 0.0, 1.0, 1.0],
                                4 => [1.0; 4],
                                _ => [1.0; 4],
                            };

                            rectangle(
                                color,
                                [
                                    position[0] + x as f64 * size,
                                    position[1] + y as f64 * size,
                                    size,
                                    size,
                                ],
                                context.transform,
                                graphics,
                            );
                        }
                    }
                }

                if r#char == ' ' {
                    position[0] += size * 2.0;
                } else {
                    position[0] += (size * array_pixel[0].len() as f64) + (1.0 * size);
                }
            }
        });
    }
}
