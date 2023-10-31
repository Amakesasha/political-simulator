use crate::*;

impl GameS {
    #[cfg(feature = "draw")]
    pub fn draw_date(&mut self, size: f64, display: &mut PistonWindow, event: &Event) {
        let date = &self.logic.date;

        if date.draw {
            let day = if date.date[0] <= 9 {
                format!("0{}", date.date[0])
            } else {
                format!("{}", date.date[0])
            };

            let month = if date.date[1] <= 9 {
                format!("0{}", date.date[1])
            } else {
                format!("{}", date.date[1])
            };

            let date = "Date: ".to_owned()
                + &day
                + "."
                + &month
                + "."
                + format!("{}", date.date[2]).as_str();

            self.context.draw_text([1.0 * size, 1.0 * size], size, date, display, event);
        }
    }
}
