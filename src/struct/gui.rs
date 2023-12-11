use crate::*;

pub mod gui {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct GuiS {
        pub(crate) button: Vec<ButtonS>,
        pub(crate) window: Vec<WindowS>,
        pub(crate) table: Vec<TableS>,
        pub(crate) path: Vec<PathS>,
    }

    pub static mut GR: bool = false;

    pub type FactsGui = (
        Vec<FactsButton>,
        Vec<FactsWindow>,
        Vec<FactsTable>,
        Vec<FactsPath>,
    );

    impl Create for GuiS {
        type Output = GuiS;
        type Facts = FactsGui;

        fn new(facts: &Self::Facts) -> Self::Output {
            GuiS {
                button: ButtonS::vec_new(&facts.0),
                window: WindowS::vec_new(&facts.1),
                table: TableS::vec_new(&facts.2),
                path: PathS::vec_new(&facts.3),
            }
        }

        fn default() -> Self::Output {
            GuiS {
                button: Vec::new(),
                window: Vec::new(),
                table: Vec::new(),
                path: Vec::new(),
            }
        }
    }
}

pub mod window {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct WindowS {
        pub(crate) name: String,
        pub(crate) draw: bool,
        pub(crate) aabb: AabbS,

        pub(crate) flooded_border: Option<bool>,
        pub(crate) color: [Color; 2],

        pub(crate) button: Vec<ButtonS>,
    }

    pub type FactsWindow = (
        String,
        bool,
        FactsAabb,
        Option<bool>,
        [Color; 2],
        Vec<FactsButton>,
    );

    impl Default for &WindowS {
        fn default() -> Self {
            Self::default()
        }
    }

    impl Create for WindowS {
        type Output = WindowS;
        type Facts = FactsWindow;

        fn new(facts: &Self::Facts) -> Self::Output {
            WindowS {
                name: String::from(&facts.0),
                draw: facts.1,
                aabb: AabbS::new(&facts.2),

                flooded_border: facts.3.clone(),
                color: facts.4,

                button: ButtonS::vec_new(&facts.5),
            }
        }

        fn default() -> Self::Output {
            WindowS {
                name: String::new(),
                draw: false,
                aabb: AabbS::default(),

                flooded_border: None,
                color: ColorR::defaulf_2(),

                button: Vec::new(),
            }
        }
    }

    impl Give for WindowS {
        type Output = WindowS;
        type ID = String;

        fn vector_give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<&Self::Output> {
            facts.iter().find(|data| data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| data.name == id)
        }

        fn hashmap_give(
            facts: &HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&Self::Output> {
            facts
                .iter()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts
                .iter_mut()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }
    }

    impl Gui for WindowS {
        type Output = WindowS;

        fn open_all_close(vec: &mut Vec<Self::Output>, meaning: &bool) {
            for window in vec {
                window.draw = *meaning;
            }
        }

        fn open_rest_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            WindowS::open_all_close(vec, &!meaning);

            if let Some(window) = WindowS::vector_give_mut(vec, name.to_string()) {
                window.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(window) = WindowS::vector_give_mut(vec, name.to_string()) {
                window.draw = *meaning;
            }
        }
    }
}

pub mod button {
    use crate::*;

    #[derive(Debug, Clone)]
    pub enum ActionE {
        OpenWindow(GuiActionE),
        OpenButton(GuiActionE),
        OpenTable(GuiActionE),
        StartFunction(String),
        Non,
    }

    #[derive(Debug, Clone)]
    pub enum GuiActionE {
        OpenAllClose(bool),
        OpenRestClose(String, bool),
        OpenOneClose(String, bool),
    }

    //

    #[derive(Debug, Clone)]
    pub struct ButtonS {
        pub(crate) name: String,
        pub(crate) draw: bool,
        pub(crate) aabb: AabbS,

        pub(crate) flooded_border: Option<bool>,
        pub(crate) color: [Color; 2],

        pub(crate) button: Option<KeyCode>,
        pub(crate) action: Vec<ActionE>,
        pub(crate) text: Option<(String, [Color; 2])>,
    }

    pub type FactsButton = (
        String,
        bool,
        FactsAabb,
        Option<bool>,
        [Color; 2],
        Option<KeyCode>,
        Vec<ActionE>,
        Option<(String, [Color; 2])>,
    );

    impl Default for &ButtonS {
        fn default() -> Self {
            Self::default()
        }
    }

    impl Create for ButtonS {
        type Output = ButtonS;
        type Facts = FactsButton;

        fn new(facts: &Self::Facts) -> Self::Output {
            ButtonS {
                name: String::from(&facts.0),
                draw: facts.1,
                aabb: AabbS::new(&facts.2),

                flooded_border: facts.3.clone(),
                color: facts.4,

                button: facts.5,
                action: facts.6.clone(),

                text: facts.7.clone(),
            }
        }

        fn default() -> Self::Output {
            ButtonS {
                name: String::new(),
                draw: false,
                aabb: AabbS::default(),

                flooded_border: None,
                color: ColorR::defaulf_2(),

                button: None,
                action: Vec::new(),

                text: None,
            }
        }
    }

    impl Give for ButtonS {
        type Output = ButtonS;
        type ID = String;

        fn vector_give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<&Self::Output> {
            facts.iter().find(|data| data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| data.name == id)
        }

        fn hashmap_give(
            facts: &HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&Self::Output> {
            facts
                .iter()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts
                .iter_mut()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }
    }

    impl Gui for ButtonS {
        type Output = ButtonS;

        fn open_all_close(vec: &mut Vec<Self::Output>, meaning: &bool) {
            for button in vec {
                button.draw = *meaning;
            }
        }

        fn open_rest_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            ButtonS::open_all_close(vec, &!meaning);

            if let Some(button) = ButtonS::vector_give_mut(vec, name.to_string()) {
                button.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(button) = ButtonS::vector_give_mut(vec, name.to_string()) {
                button.draw = *meaning;
            }
        }
    }
}

pub mod table {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct TableS {
        pub(crate) name: String,
        pub(crate) draw: bool,
        pub(crate) aabb: AabbS,

        pub(crate) flooded_border: Option<bool>,
        pub(crate) color: [Color; 2],

        pub(crate) indentation: u16,
        pub(crate) cells: Vec<Vec<(String, [Color; 2],)>>,
    }

    pub type FactsTable = (
        String,
        bool,
        FactsAabb,
        Option<bool>,
        [Color; 2],
        u16,
        Vec<Vec<(String, [Color; 2],)>>,
    );

    impl Default for &TableS {
        fn default() -> Self {
            Self::default()
        }
    }

    impl Create for TableS {
        type Output = TableS;
        type Facts = FactsTable;

        fn new(facts: &Self::Facts) -> Self::Output {
            TableS {
                name: String::from(&facts.0),
                draw: facts.1,
                aabb: AabbS::new(&facts.2),

                flooded_border: facts.3.clone(),
                color: facts.4,

                indentation: facts.5,
                cells: facts.6.clone(),
            }
        }

        fn default() -> Self::Output {
            TableS {
                name: String::new(),
                draw: false,
                aabb: AabbS::default(),

                flooded_border: None,
                color: ColorR::defaulf_2(),

                indentation: 0,
                cells: Vec::new(),
            }
        }
    }

    impl Give for TableS {
        type Output = TableS;
        type ID = String;

        fn vector_give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<&Self::Output> {
            facts.iter().find(|data| data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| data.name == id)
        }

        fn hashmap_give(
            facts: &HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&Self::Output> {
            facts
                .iter()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts
                .iter_mut()
                .find(|(_, data)| data.name == id)
                .map(|(_, data)| data)
        }
    }

    impl Gui for TableS {
        type Output = TableS;

        fn open_all_close(vec: &mut Vec<Self::Output>, meaning: &bool) {
            for table in vec {
                table.draw = *meaning;
            }
        }

        fn open_rest_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            TableS::open_all_close(vec, &!meaning);

            if let Some(table) = TableS::vector_give_mut(vec, name.to_string()) {
                table.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(table) = TableS::vector_give_mut(vec, name.to_string()) {
                table.draw = *meaning;
            }
        }
    }
}

pub mod path {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct PathS {
        pub(crate) code: KeyCode,
        pub(crate) name: Result<String, [String; 2]>,
    }

    pub type FactsPath = (KeyCode, Result<String, [String; 2]>);

    impl Create for PathS {
        type Output = PathS;
        type Facts = FactsPath;

        fn new(facts: &Self::Facts) -> Self::Output {
            PathS {
                code: facts.0,
                name: facts.1.clone(),
            }
        }

        fn default() -> Self::Output {
            PathS {
                code: KeyCode::Esc,
                name: Ok(String::new()),
            }
        }
    }

    impl Give for PathS {
        type Output = PathS;
        type ID = KeyCode;

        fn vector_give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<&Self::Output> {
            facts.iter().find(|data| data.code == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| data.code == id)
        }

        fn hashmap_give(
            facts: &HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&Self::Output> {
            facts
                .iter()
                .find(|(_, data)| data.code == id)
                .map(|(_, data)| data)
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts
                .iter_mut()
                .find(|(_, data)| data.code == id)
                .map(|(_, data)| data)
        }
    }
}
