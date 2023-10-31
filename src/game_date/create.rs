use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        let logic = {
            let country = {
                let mut vector = Vec::new();

                vector.push({
                    (
                        "Country".to_string(),
                        [
                            (0.0, 0.0, 1, 5.0, [1000.0, 850.0, 675.0]), 
                            (0.0, 0.0, 1, 2.5, [500.0, 450.0, 250.0]),
                            (0.0, 0.0, 1, 1.25, [2500.0, 1250.0, 1000.0]),
                        ],
                    )
                });

                vector
            };

            let date = (true, [02, 10, 2009], [1, 0, 0]);

            (date, country)
        };

        let context = {
            let play = true;
            let stop = false;

            let button = [
                (Button::Keyboard(Key::W), true),
                (Button::Keyboard(Key::A), true),
                (Button::Keyboard(Key::S), true),
                (Button::Keyboard(Key::D), true),

                (Button::Keyboard(Key::D1), true),
                (Button::Keyboard(Key::D2), true),
                (Button::Keyboard(Key::D3), true),

                (Button::Keyboard(Key::Escape), true),
            ];

            (play, stop, button)
        };

        let gui = {
            let button = Vec::new();
            let window = Vec::new();

            (button, window)
        };

        return (logic, context, gui);
    }
}
