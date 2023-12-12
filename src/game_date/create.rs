use crate::*;

impl GameS {
    pub fn give_date() -> FactsGame {
        let logic = LogicS::give_date();
        let gui = GuiS::give_date();

        return (logic, gui);
    }
}

pub mod logic {
    use crate::*;

    impl LogicS {
        pub fn give_date() -> FactsLogic {
            let date = DateS::give_date();

            let name = String::from("Country");

            let password = vec!["123456".to_string(), "wasd".to_string()];

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
                1 - price
                2 - number_of_factory
                3 - production_1_factory
            */

            vector.push({
                (
                    "Country".to_string(),
                    [
                        [
                            (0.0, 0.0, 10, 5.0),
                            (0.0, 0.0, 5, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                        [
                            (0.0, 0.0, 10, 5.0),
                            (0.0, 0.0, 5, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                    ],
                )
            });

            vector.push({
                (
                    "Test".to_string(),
                    [
                        [
                            (0.0, 0.0, 3, 5.0),
                            (0.0, 0.0, 2, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                        [
                            (0.0, 0.0, 3, 5.0),
                            (0.0, 0.0, 2, 2.5),
                            (0.0, 0.0, 1, 1.25),
                            (0.0, 0.0, 1, 0.25),
                        ],
                    ],
                )
            });

            vector
        }
    }
}

pub mod gui {
    use crate::*;

    impl GuiS {
        pub fn give_date() -> FactsGui {
            let button = ButtonS::give_date();
            let window = WindowS::give_date();
            let table = TableS::give_date();
            let path = PathS::give_date();

            (button, window, table, path)
        }
    }

    #[cfg(feature = "button")]
    impl ButtonS {
        pub fn give_date() -> Vec<FactsButton> {
            let mut vector = Vec::new();

            /*vector.push({
                (
                    String::from("qwerty"),
                    true,
                    ([20, 30], [10, 20]),
                    false,
                    [Color::White, Color::Red],
                    None,
                    ActionE::Non,
                    Some((
                        String::from("Hello!!\nI'm_Sasha\n X_Y_U"),
                        [Color::White, Color::Red],
                    )),
                )
            });*/

            vector
        }
    }

    #[cfg(not(feature = "button"))]
    impl ButtonS {
        pub fn give_date() -> Vec<FactsButton> {
            Vec::new()
        }
    }

    //

    #[cfg(feature = "window")]
    impl WindowS {
        pub fn give_date() -> Vec<FactsWindow> {
            let mut vector = Vec::new();

            vector.push({
                (
                    String::from("top_line"),
                    true,
                    ([1, 1], [150, 9]),
                    Some(true),
                    [Color::Green, Color::Blue],
                    {
                        let mut vector = Vec::new();

                        vector.push({
                            (
                                String::from("date"),
                                true,
                                ([2, 2], [14, 5]),
                                Some(true),
                                [Color::Cyan, Color::Black],
                                None,
                                Vec::new(),
                                Some((String::from("\n 02.10.2009"), [Color::White, Color::Black])),
                            )
                        });

                        vector
                    },
                )
            });

            vector.push({
                (
                    String::from("navigation_bar"),
                    true,
                    ([1, 11], [25, 35]),
                    Some(true),
                    [Color::Green, Color::Blue],
                    {
                        let mut vector = Vec::new();

                        vector.push({
                            (
                                String::from("resourse"),
                                true,
                                ([2, 2], [21, 5]),
                                Some(true),
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('1')),
                                vec![
                                    ActionE::OpenWindow(GuiActionE::OpenOneClose(
                                        String::from("resourse"),
                                        true,
                                    )),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("resourse"),
                                        true,
                                    )),
                                ],
                                Some((
                                    String::from("\n   Resourse (1)"),
                                    [Color::White, Color::Black],
                                )),
                            )
                        });

                        vector.push({
                            (
                                String::from("electricity"),
                                true,
                                ([2, 8], [21, 5]),
                                Some(true),
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('2')),
                                vec![
                                    ActionE::OpenWindow(GuiActionE::OpenOneClose(
                                        String::from("electricity"),
                                        true,
                                    )),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("electricity"),
                                        true,
                                    )),
                                ],
                                Some((
                                    String::from("\n  Electricity (2)"),
                                    [Color::White, Color::Black],
                                )),
                            )
                        });

                        vector.push({
                            (
                                String::from("trade"),
                                true,
                                ([2, 14], [21, 5]),
                                Some(true),
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('3')),
                                vec![
                                    ActionE::OpenWindow(GuiActionE::OpenOneClose(
                                        String::from("trade"),
                                        true,
                                    )),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("trade"),
                                        true,
                                    )),
                                ],
                                Some((
                                    String::from("\n     Trade (3)"),
                                    [Color::White, Color::Black],
                                )),
                            )
                        });

                        vector
                    },
                )
            });

            vector.push({
                (
                    String::from("resourse"),
                    true,
                    ([27, 11], [124, 35]),
                    Some(true),
                    [Color::Green, Color::Blue],
                    {
                        let mut vector = Vec::new();

                        vector.push({
                            (
                                String::from("name_resourse"),
                                false,
                                ([2, 2], [12, 5]),
                                Some(true),
                                [Color::Cyan, Color::Black],
                                None,
                                Vec::new(),
                                Some((String::from("\n  Resourse "), [Color::White, Color::Black])),
                            )
                        });

                        vector
                    },
                )
            });

            vector
        }
    }

    #[cfg(not(feature = "window"))]
    impl WindowS {
        pub fn give_date() -> Vec<FactsWindow> {
            Vec::new()
        }
    }

    //

    #[cfg(feature = "table")]
    impl TableS {
        pub fn give_date() -> Vec<FactsTable> {
            let mut vector = Vec::new();

            vector.push({
                (
                    String::from("resourse"),
                    true,
                    [29, 13],
                    Some(true),
                    [Color::Cyan, Color::Black],
                    1,
                    vec![
                        (
                            ([0, 0], [13, 5]),
                            vec![
                                (String::from("\n Resourse "), [Color::White, Color::Black]),
                                (String::from("AA"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                (String::from("\n Resourse "), [Color::White, Color::Black]),
                                (String::from("AA"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                (String::from("\n Resourse "), [Color::White, Color::Black]),
                                (String::from("AA"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                (String::from("\n Resourse "), [Color::White, Color::Black]),
                                (String::from("AA"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                (String::from("\n Resourse "), [Color::White, Color::Black]),
                                (String::from("AA"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                                (String::from("ZZ"), [Color::White, Color::Black]),
                            ],
                        )
                    ],
                )
            });

            vector
        }
    }

    #[cfg(not(feature = "table"))]
    impl TableS {
        pub fn give_date() -> Vec<FactsTable> {
            Vec::new()
        }
    }

    impl PathS {
        pub fn give_date() -> Vec<FactsPath> {
            let mut vector = Vec::new();

            vector.push({
                (
                    KeyCode::Char('1'),
                    Err([String::from("main"), String::from("resourse")]),
                )
            });

            vector.push({
                (
                    KeyCode::Char('2'),
                    Err([String::from("navigation_bar"), String::from("electricity")]),
                )
            });

            vector
        }
    }
}
