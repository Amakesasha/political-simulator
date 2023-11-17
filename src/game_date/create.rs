use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        let logic = {
            let date = (true, [02, 10, 2009], [1, 0, 0]);
            let name = String::from("");

            let password = vec!["123456".to_string(), "wasd".to_string(), "qwerty".to_string()];

            let country = {
                let mut vector = Vec::new();

                vector.push({
                    (
                        "Country".to_string(),
                        [
                            (0.0, 0.0, 10, 5.0),
                            (0.0, 0.0, 5, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                        [
                            (10, 5.0),
                            (5, 2.5),
                            (1, 1.25),
                            (1, 0.25),
                        ],
                    )
                });

                vector.push({
                    (
                        "Test".to_string(),
                        [
                            (0.0, 0.0, 3, 5.0),
                            (0.0, 0.0, 2, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                        [
                            (10, 5.0),
                            (5, 2.5),
                            (1, 1.25),
                            (1, 0.25),
                        ],
                    )
                });

                vector.push({
                    (
                        "Test_1".to_string(),
                        [
                            (50000.0, 0.0, 100, 5.0),
                            (50000.0, 0.0, 100, 2.5),
                            (50000.0, 0.0, 100, 1.25),
                            (500000.0, 0.0, 100, 0.25),
                        ],
                        [
                            (10, 5.0),
                            (5, 2.5),
                            (1, 1.25),
                            (1, 0.25),
                        ],
                    )
                });

                vector
            };

            let resources__for_construction_factory = [
                [1000.0, 850.0, 675.0, 0.0],
                [500.0, 450.0, 250.0, 0.0], 
                [2500.0, 1250.0, 1000.0, 0.0],
                [7500.0, 3450.0, 2500.0, 0.0],
            ];

            let resources__for_construction_power_stantion = [
                [1000.0, 850.0, 675.0, 0.0],
                [500.0, 450.0, 250.0, 0.0], 
                [2500.0, 1250.0, 1000.0, 0.0],
                [7500.0, 3450.0, 2500.0, 0.0],
            ];

            (
                date, 
                name, 
                password, 
                country, 
                resources__for_construction_factory,
                resources__for_construction_power_stantion
            )
        };

        return logic;
    }
}
