use crate::*;

impl GameS {
    pub fn give_date_hash_map() -> HashMap<(String, String), GameS> {
        let mut hash_map = HashMap::new();

        hash_map.insert(
            ("Qwerty".to_string(), "123456".to_string()),
            GameS {
                logic: LogicS {
                    countries: CountryS::vec_new(
                        vec![
                            (
                                "Country".to_string(),
                                [
                                    (0.0, 0.0, 1, 5.0, [1000.0, 850.0, 675.0]),
                                    (0.0, 0.0, 1, 2.5, [500.0, 450.0, 250.0]),
                                    (0.0, 0.0, 1, 1.25, [2500.0, 1250.0, 1000.0]),
                                ],
                            ),
                        ]
                    ),
                    date: DateS::new((true, [22, 4, 2009], [1, 0, 0])),
                }
            }
        );

        hash_map
    }

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

        return (logic);
    }
}
