use crate::*;

pub static SIZE_BLOCK: f64 = 1.0;

impl GameS {
    #[cfg(feature = "draw")]
    pub fn draw(&mut self, display: &mut PistonWindow, event: &Event) {
        let font = &self.context.font;

        display.draw_2d(event, |_, graphics, _| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
        });

        self.logic.date.draw(5.0, &self.context.font, display, event);

        self.gui.draw(&self.context.font, display, event);

        if let Some(country) = CountryS::give(&self.logic.countries, "Country".to_string()) {
            let size = 5.0;
            
            let tree = &country.storage.tree;
            let iron = &country.storage.iron;
            let concrete = &country.storage.concrete;

            let data_0 = format!("Concrete: {}", concrete.quantity);
            let data_1 = format!("Tree:     {}", tree.quantity);
            let data_2 = format!("Iron:     {}", iron.quantity);
            
            font.draw([1.0 * size, 30.0 * size], size, data_0, display, event);
            font.draw([1.0 * size, 40.0 * size], size, data_1, display, event);
            font.draw([1.0 * size, 50.0 * size], size, data_2, display, event);


            let data_0 = format!("{} | {}", concrete.number_of_factories, concrete.production_1_plant);
            let data_1 = format!("{} | {}", tree.number_of_factories, tree.production_1_plant);
            let data_2 = format!("{} | {}", iron.number_of_factories, iron.production_1_plant);

            font.draw([100.0 * size, 30.0 * size], size, data_0, display, event);
            font.draw([100.0 * size, 40.0 * size], size, data_1, display, event);
            font.draw([100.0 * size, 50.0 * size], size, data_2, display, event);
        }
    }
}
