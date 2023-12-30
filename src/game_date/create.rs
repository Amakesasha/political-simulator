use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        let logic = LogicS::give_date();

        return logic;
    }
}

impl LogicS {
    pub fn give_date() -> FactsLogic {
        let date = DateS::give_date();

        let name = [String::from("123456"), String::from("Country")];

        let password = vec![String::from("123456"), String::from("wasd")];

        let country = CountryS::give_date();

        let resources_for_construction = [
            [
                [1000.0, 850.0, 675.0, 0.0],
                [500.0, 450.0, 250.0, 0.0],
                [2500.0, 1250.0, 1000.0, 0.0],
                [7500.0, 3450.0, 2500.0, 0.0],
            ],
            [
                [1000.0, 850.0, 675.0, 0.0],
                [500.0, 450.0, 250.0, 0.0],
                [2500.0, 1250.0, 1000.0, 0.0],
                [7500.0, 3450.0, 2500.0, 0.0],
            ],
            [[0.0; 4]; 4],
        ];

        (date, name, password, country, resources_for_construction)
    }
}

impl DateS {
    pub fn give_date() -> FactsDate {
        (true, [02, 10, 2009], [1, 0, 0])
    }
}

impl CountryS {
    pub fn give_date() -> Vec<FactsCountry> {
        let mut vector = Vec::new();

        /*
            0 - quantity
            1 - storage
            2 - number_of_factory
            3 - production_1_factory
        */

        vector.push({
            (
                String::from("Country"),
                [
                    [
                        (100.0, 5, 100.0, 10, 5.0),
                        (100.0, 5, 100.0, 5, 2.5),
                        (100.0, 5, 100.0, 1, 1.25),
                        (100.0, 5, 100.0, 1, 0.25),
                    ],
                    [
                        (0.0, 5, 100.0, 10, 5.0),
                        (0.0, 5, 100.0, 5, 2.5),
                        (0.0, 5, 100.0, 1, 1.25),
                        (0.0, 5, 100.0, 1, 0.25),
                    ],
                ],
            )
        });

        vector.push({
            (
                String::from("Test"),
                [
                    [
                        (1000.0, 5, 100.0, 3, 5.0),
                        (1000.0, 5, 100.0, 2, 2.5),
                        (1000.0, 5, 100.0, 1, 1.25),
                        (1000.0, 5, 100.0, 1, 0.25),
                    ],
                    [
                        (0.0, 5, 100.0, 3, 5.0),
                        (0.0, 5, 100.0, 2, 2.5),
                        (0.0, 5, 100.0, 1, 1.25),
                        (0.0, 5, 100.0, 1, 0.25),
                    ],
                ],
            )
        });

        vector
    }
}
