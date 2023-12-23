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
                1 - number_of_factory
                2 - production_1_factory
            */

            vector.push({
                (
                    String::from("Country"),
                    [
                        [
                            (100000.0, 10, 5.0),
                            (100000.0, 5, 2.5),
                            (100000.0, 1, 1.25),
                            (100000.0, 1, 0.25),
                        ],
                        [
                            (0.0, 10, 5.0),
                            (0.0, 5, 2.5),
                            (0.0, 1, 1.25),
                            (0.0, 1, 0.25),
                        ],
                    ],
                )
            });

            vector.push({
                (
                    String::from("Test"),
                    [
                        [
                            (100000.0, 3, 5.0),
                            (100000.0, 2, 2.5),
                            (100000.0, 1, 1.25),
                            (100000.0, 1, 0.25),
                        ],
                        [
                            (0.0, 3, 5.0),
                            (0.0, 2, 2.5),
                            (0.0, 1, 1.25),
                            (0.0, 1, 0.25),
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
            let button: Vec<FactsButton> = ButtonS::give_date();
            let window: Vec<FactsWindow> = WindowS::give_date();
            let table: Vec<FactsTable> = TableS::give_date();
            let path: Vec<FactsPath> = PathS::give_date();
            let gui_render: FactsGuiRender = GuiRenderS::give_date();

            (button, window, table, path, gui_render)
        }
    }

    //

    #[cfg(feature = "window")]
    impl WindowS {
        pub fn give_date() -> Vec<FactsWindow> {
            let mut vector: Vec<FactsWindow> = Vec::new();

            vector.push({
                (
                    String::from("top_line"),
                    true,
                    ([1, 1], [150, 9]),
                    [true; 2],
                    [Color::Green, Color::Blue],

                    #[cfg(feature = "button")] {
                        let mut vector: Vec<FactsButton> = Vec::new();

                        vector.push({
                            (
                                String::from("date"),
                                true,
                                ([134, 2], [14, 5]),
                                [true; 2],
                                [Color::Cyan, Color::Black],
                                None,
                                Vec::new(),
                                Some((
                                    [Color::White, Color::Black],
                                    (String::from("\n 02.10.2009")),
                                )),
                            )
                        });

                        vector
                    },
                    #[cfg(not(feature = "button"))] {
                        Vec::new()
                    },
                )
            });

            vector.push({
                (
                    String::from("navigation_bar"),
                    true,
                    ([1, 11], [25, 35]),
                    [true; 2],
                    [Color::Green, Color::Blue],
                    #[cfg(feature = "button")] {
                        let mut vector: Vec<FactsButton> = Vec::new();

                        vector.push({
                            (
                                String::from("resourse"),
                                true,
                                ([2, 2], [21, 5]),
                                [true; 2],
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('1')),
                                vec![
                                    ActionE::OpenTable(GuiActionE::OpenAllClose(false)),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("resourse"),
                                        true,
                                    )),
                                ],
                                Some((
                                    [Color::White, Color::Black],
                                    String::from("\n   Resourse (1)"),
                                )),
                            )
                        });

                        vector.push({
                            (
                                String::from("electricity"),
                                true,
                                ([2, 8], [21, 5]),
                                [true; 2],
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('2')),
                                vec![
                                    ActionE::OpenTable(GuiActionE::OpenAllClose(false)),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("electricity"),
                                        true,
                                    )),
                                ],
                                Some((
                                    [Color::White, Color::Black],
                                    String::from("\n  Electricity (2)"),
                                )),
                            )
                        });

                        vector.push({
                            (
                                String::from("trade"),
                                true,
                                ([2, 14], [21, 5]),
                                [true; 2],
                                [Color::Cyan, Color::Black],
                                Some(KeyCode::Char('3')),
                                vec![
                                    ActionE::OpenTable(GuiActionE::OpenAllClose(false)),
                                    ActionE::OpenTable(GuiActionE::OpenOneClose(
                                        String::from("trade"),
                                        true,
                                    )),
                                ],
                                Some((
                                    [Color::White, Color::Black],
                                    String::from("\n     Trade (3)"),
                                )),
                            )
                        });

                        vector
                    },
                    #[cfg(not(feature = "button"))] {
                        Vec::new()
                    },
                )
            });

            vector.push({
                (
                    String::from("main"),
                    true,
                    ([27, 11], [124, 35]),
                    [true; 2],
                    [Color::Green, Color::Blue],
                    Vec::new(),
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

    #[cfg(feature = "button")]
    impl ButtonS {
        pub fn give_date() -> Vec<FactsButton> {
            let mut vector: Vec<FactsButton> = Vec::new();

            vector.push({
                (
                    String::new(),
                    false,
                    ([0, 0], [0, 0]),
                    [false; 2],
                    ColorR::defaulf_2(),
                    None,
                    Vec::new(),
                    None,
                )
            });

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

    #[cfg(feature = "table")]
    impl TableS {
        pub fn give_date() -> Vec<FactsTable> {
            let mut vector: Vec<FactsTable> = Vec::new();

            vector.push({
                (
                    String::from("resourse"),
                    true,
                    [29, 13],
                    [true; 2],
                    [Color::Cyan, Color::Black],
                    1,
                    vec![
                        (
                            ([0, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Resourse ")),
                                ([Color::White, Color::Black], String::from("\n  Concrete")),
                                ([Color::White, Color::Black], String::from("\n   Tree")),
                                ([Color::White, Color::Black], String::from("\n   Iron")),
                                ([Color::White, Color::Black], String::from("\n   Rubber")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                    ],
                )
            });

            vector.push({
                (
                    String::from("electricity"),
                    false,
                    [29, 13],
                    [true; 2],
                    [Color::Magenta, Color::Black],
                    1,
                    vec![
                        (
                            ([0, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Resourse ")),
                                ([Color::White, Color::Black], String::from("\n ")),
                                ([Color::White, Color::Black], String::from("\n ")),
                                ([Color::White, Color::Black], String::from("\n ")),
                                ([Color::White, Color::Black], String::from("\n ")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
                        (
                            ([14, 0], [13, 5]),
                            vec![
                                ([Color::White, Color::Black], String::from("\n Quantity ")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                                ([Color::White, Color::Black], String::from("\n")),
                            ],
                        ),
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

    //

    impl PathS {
        pub fn give_date() -> Vec<FactsPath> {
            let mut vector: Vec<FactsPath> = Vec::new();

            vector.push({
                (
                    KeyCode::Char('1'),
                    Err([String::from("navigation_bar"), String::from("resourse")]),
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

    impl GuiRenderS {
        pub fn give_date() -> FactsGuiRender {
            (false, Ok(false), Ok(false), Ok(false))
        }
    }
}
