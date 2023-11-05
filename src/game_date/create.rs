use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        let logic = {
            let password = vec!["123456".to_string()];

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

            (date, password, country, String::new())
        };

        return (logic);
    }
}
