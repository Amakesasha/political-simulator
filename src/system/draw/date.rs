use crate::*;

impl DateS {
    pub fn draw(&self, size: f64, font: &FontS, display: &mut PistonWindow, event: &Event) {
        if self.draw {
            let day = if self.date[0] <= 9 {
                format!("0{}", self.date[0])
            } else {
                format!("{}", self.date[0])
            };

            let month = if self.date[1] <= 9 {
                format!("0{}", self.date[1])
            } else {
                format!("{}", self.date[1])
            };

            let date = "Date: ".to_owned()
                + &day
                + "."
                + &month
                + "."
                + format!("{}", self.date[2]).as_str();

            font.draw([1.0 * size, 1.0 * size], size, date, display, event);
        }
    }
}
