use crate::*;

pub static SIZE_BLOCK: f64 = 1.0;

impl GameS {
    #[cfg(feature = "draw")]
    pub fn draw(&mut self, display: &mut PistonWindow, event: &Event) {
        let font = &self.context.font;

        draw::clean(display, Some(event));

        self.logic.date.draw(5.0, &font, display, event);

        self.gui.draw(&font, display, event);

        self.logic.draw(&font, display, event);
    }
}

pub mod draw {
    use crate::*;

    pub fn clean(display: &mut PistonWindow, event: Option<&Event>) {
        if let Some(event) = event {
            display.draw_2d(event, |_, graphics, _| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);
            });
        } else {
            if let Some(event) = display.next() {
                display.draw_2d(&event, |_, graphics, _| {
                    clear([0.0, 0.0, 0.0, 1.0], graphics);
                });
            } else {
                return;
            }
        }
    } 
}