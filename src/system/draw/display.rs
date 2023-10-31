use crate::*;

#[cfg(feature = "draw")]
pub fn display_lear(display: &mut PistonWindow, event: Option<&Event>) {
    if let Some(event) = event {
        display.draw_2d(event, |_, graphics, _| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
        });
    } else {
        if let Some(event) = display.next() {
            display.draw_2d(&event, |_, graphics, _| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);
            });

            return;
        }
    }
}
