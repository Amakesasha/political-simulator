pub mod gui {
    use crate::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct GuiS {
        pub(crate) window: Vec<WindowS>,
        pub(crate) button: Vec<ButtonS>,
        pub(crate) table: Vec<TableS>,
    }

    pub type GuiFacts = (Vec<WindowFacts>, Vec<ButtonFacts>, Vec<TableFacts>);

    impl Create for GuiS {
        type Output = GuiS;
        type Facts = GuiFacts;

        fn new(facst: &Self::Facts) -> Self::Output {
            GuiS {
                window: WindowS::vec_new(&facst.0),
                button: ButtonS::vec_new(&facst.1),
                table: TableS::vec_new(&facst.2),
            }
        }

        fn default() -> Self::Output {
            GuiS {
                window: Vec::new(),
                button: Vec::new(),
                table: Vec::new(),
            }
        }

        fn default_facts() -> Self::Facts {
            (Vec::new(), Vec::new(), Vec::new())
        }
    }

    pub type AabbT = [[usize; 2]; 2];
}

//

pub mod window {
    use crate::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct WindowS {
        pub(crate) name: String,

        pub(crate) aabb: AabbT,
        pub(crate) draw: bool,

        pub(crate) color_main: u32,
        pub(crate) color_border: Option<u32>,

        pub(crate) line_width: usize,

        pub(crate) button: Vec<ButtonS>,
    }

    pub type WindowFacts = (String, AabbT, bool, u32, Option<u32>, usize, Vec<ButtonFacts>);

    impl Create for WindowS {
        type Output = WindowS;
        type Facts = WindowFacts;

        fn new(facst: &Self::Facts) -> Self::Output {
            WindowS {
                name: facst.0.clone(),

                aabb: facst.1,
                draw: facst.2,

                color_main: facst.3,
                color_border: facst.4,

                line_width: facst.5,

                button: ButtonS::vec_new(&facst.6),
            }
        }

        fn default() -> Self::Output {
            WindowS {
                name: String::new(),

                aabb: [[0; 2]; 2],
                draw: false,

                color_main: 0,
                color_border: None,

                line_width: 0,

                button: Vec::new(),
            }
        }

        fn default_facts() -> Self::Facts {
            (String::new(), [[0; 2]; 2], false, 0, None, 0, Vec::new())
        }
    }
}

//

pub mod button {
    use crate::*;

    // OorC - Open or Close | OC - Open Close

    #[derive(Debug, Clone, PartialEq)]
    pub enum ActionE {
        WindowOC(bool, OorCE),
        ButtonOC(bool, OorCE),
        TableOC(bool, OorCE),
        StartFunction(String),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum OorCE {
        Full,
        One(String),
        Some(Vec<String>),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ButtonS {
        pub(crate) name: String,

        pub(crate) aabb: AabbT,
        pub(crate) draw: bool,

        pub(crate) color_main: u32,
        pub(crate) color_border: Option<u32>,

        pub(crate) line_width: usize,

        pub(crate) key: Key,
        pub(crate) actions: Vec<ActionE>,
    }

    pub type ButtonFacts = (String, AabbT, bool, u32, Option<u32>, usize, Key, Vec<ActionE>);

    impl Create for ButtonS {
        type Output = ButtonS;
        type Facts = ButtonFacts;

        fn new(facst: &Self::Facts) -> Self::Output {
            ButtonS {
                name: facst.0.clone(),

                aabb: facst.1,
                draw: facst.2,

                color_main: facst.3,
                color_border: facst.4.clone(),

                line_width: facst.5,

                key: facst.6,
                actions: facst.7.clone(),
            }
        }

        fn default() -> Self::Output {
            ButtonS {
                name: String::new(),

                aabb: [[0; 2]; 2],
                draw: false,

                color_main: 0,
                color_border: None,

                line_width: 0,

                key: Key::Escape,
                actions: Vec::new(),
            }
        }

        fn default_facts() -> Self::Facts {
            (
                String::new(),
                [[0; 2]; 2],
                false,
                0,
                None,
                0,
                Key::Escape,
                Vec::new(),
            )
        }
    }
}

//

pub mod table {
    use crate::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct TableS {
        pub(crate) name: String,

        pub(crate) position: [usize; 2],
        pub(crate) draw: bool,

        pub(crate) cells: Vec<Vec<CellT>>,

        pub(crate) color_cells_main: u32,
        pub(crate) color_cells_border: Option<u32>,

        pub(crate) color_full_border: Option<u32>,

        pub(crate) distance: usize,
    }

    pub type CellT = ([usize; 2], String);

    pub type TableFacts = (
        String,
        [usize; 2],
        bool,
        Vec<Vec<CellT>>,
        u32,
        Option<u32>,
        Option<u32>,
        usize,
    );

    impl Create for TableS {
        type Output = TableS;
        type Facts = TableFacts;

        fn new(facst: &Self::Facts) -> Self::Output {
            TableS {
                name: facst.0.clone(),

                position: facst.1,
                draw: facst.2,

                cells: facst.3.clone(),

                color_cells_main: facst.4,
                color_cells_border: facst.5.clone(),

                color_full_border: facst.6.clone(),

                distance: facst.7,
            }
        }

        fn default() -> Self::Output {
            TableS {
                name: String::new(),

                position: [0; 2],
                draw: false,

                cells: Vec::new(),

                color_cells_main: 0,
                color_cells_border: None,

                color_full_border: None,

                distance: 0,
            }
        }

        fn default_facts() -> Self::Facts {
            (String::new(), [0; 2], false, Vec::new(), 0, None, None, 0)
        }
    }
}
