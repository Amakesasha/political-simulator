use crate::*;

impl GuiS {
    #[cfg(feature = "draw")]
    pub fn draw(&self, font: &FontS, display: &mut PistonWindow, event: &Event) {
        GuiButtonS::draw(&self.button, None, font, display, event);
        GuiWindowS::draw(&self.window, None, font, display, event);
    }
}

impl GuiButtonS {
    #[cfg(feature = "draw")]
    pub fn draw(
        vector: &Vec<GuiButtonS>,
        aabb: Option<&AabbS>,
        font: &FontS,
        display: &mut PistonWindow,
        event: &Event,
    ) {
        for button in vector {
            if button.draw {
                if let Some(ref aabb) = aabb {
                    texture_draw(&button.aabb, Some(&aabb), button.texture, display, event);
                } else {
                    texture_draw(&button.aabb, None, button.texture, display, event);
                }

                for text in &button.text {
                    font.draw(
                        [
                            button.aabb.position.x + text.aabb.position.x,
                            button.aabb.position.y + text.aabb.position.y,
                        ],
                        text.aabb.size.width,
                        text.text.clone(),
                        display,
                        event,
                    );
                }
            }
        }
    }
}

impl GuiWindowS {
    #[cfg(feature = "draw")]
    pub fn draw(
        vector: &Vec<GuiWindowS>,
        aabb: Option<AabbS>,
        font: &FontS,
        display: &mut PistonWindow,
        event: &Event,
    ) {
        for window in vector {
            if window.draw {
                if let Some(ref aabb) = aabb {
                    texture_draw(&window.aabb, Some(aabb), window.texture, display, event);
                } else {
                    texture_draw(&window.aabb, None, window.texture, display, event);
                }

                for text in &window.text {
                    font.draw(
                        [
                            window.aabb.position.x + text.aabb.position.x,
                            window.aabb.position.y + text.aabb.position.y,
                        ],
                        text.aabb.size.width,
                        text.text.clone(),
                        display,
                        event,
                    );
                }

                GuiButtonS::draw(&window.button, Some(&window.aabb), font, display, event);
            }
        }
    }
}

pub fn texture_draw(
    aabb_0: &AabbS,
    aabb_1: Option<&AabbS>,
    texture: TextureT,
    display: &mut PistonWindow,
    event: &Event,
) {
    display.draw_2d(event, |context, graphics, _| {
        if let Some(aabb_1) = aabb_1 {
            let aabb = AabbS::aabb_add_aabb(&aabb_0, &aabb_1);

            rectangle(
                texture,
                [
                    aabb.position.x,
                    aabb.position.y,
                    aabb.size.width,
                    aabb.size.height,
                ],
                context.transform,
                graphics,
            );
        } else {
            rectangle(
                texture,
                [
                    aabb_0.position.x,
                    aabb_0.position.y,
                    aabb_0.size.width,
                    aabb_0.size.height,
                ],
                context.transform,
                graphics,
            );
        }
    });
}
