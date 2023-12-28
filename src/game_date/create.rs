use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        LogicS::give_date()
    }
}

impl LogicS {
    pub fn give_date() -> FactsLogic {
        let data = DateS::give_date();

        let country = [String::from("123456"), String::from("Country")];

        let password = vec![
            "123456".to_string(), 
            "wasd".to_string()
        ];

        let countries = CountryS::give_date();

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
            [[0.0; 4]; 4]
        ];

        (
            data,
            country,
            password,
            countries,
            resources_for_construction
        )
    }
}

impl DateS {
    pub fn give_date() -> FactsDate {
        (true, [02, 10, 2009], [1, 0, 0])
    }
}

impl CountryS {
    pub fn give_date() -> Vec<FactsCountry> {
        let mut vector: Vec<FactsCountry> = Vec::new();

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
                        (0.0, 5, 100.0, 1, 0.25)
                    ],
                ],
            )
        });

        vector
    }
}
