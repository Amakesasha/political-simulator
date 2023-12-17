use crate::*;

#[cfg(feature = "gui")]
pub mod gui {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct GuiS {
        #[cfg(feature = "button")]
        pub(crate) button: Vec<ButtonS>,
        #[cfg(feature = "window")]
        pub(crate) window: Vec<WindowS>,
        #[cfg(feature = "table")]
        pub(crate) table: Vec<TableS>,
        pub(crate) path: Vec<PathS>,
        pub(crate) gui_render: GuiRenderS,
    }

    pub type FactsGui = (
        Vec<FactsButton>,
        Vec<FactsWindow>,
        Vec<FactsTable>,
        Vec<FactsPath>,
        FactsGuiRender,
    );

    impl Create for GuiS {
        type Output = GuiS;
        type Facts = FactsGui;

        fn new(facts: &Self::Facts) -> Self::Output {
            GuiS {
                #[cfg(feature = "button")]
                button: ButtonS::vec_new(&facts.0),
                #[cfg(feature = "window")]
                window: WindowS::vec_new(&facts.1),
                #[cfg(feature = "table")]
                table: TableS::vec_new(&facts.2),
                path: PathS::vec_new(&facts.3),
                gui_render: GuiRenderS::new(&facts.4),
            }
        }

        fn default() -> Self::Output {
            GuiS {
                #[cfg(feature = "button")]
                button: Vec::new(),
                #[cfg(feature = "window")]
                window: Vec::new(),
                #[cfg(feature = "table")]
                table: Vec::new(),
                path: Vec::new(),
                gui_render: GuiRenderS::default()
            }
        }
    }

    impl Control for GuiS {
        type Facts = LogicS;

        fn update(&mut self, facts: &Self::Facts) {
            if let Some(country) = CountryS::hashmap_give(&facts.countries, &facts.name_country[1], true) {
                #[cfg(feature = "table")]
                if let Some(table) =
                    TableS::vector_give_mut(&mut self.table, &String::from("resourse"))
                {
                    if table.draw {
                        table.update(&vec![(
                            [1, 1],
                            format!("\n  {}", country.storage[0].0[0].quantity),
                        )]);
                        table.update(&vec![(
                            [1, 2],
                            format!("\n  {}", country.storage[0].0[1].quantity),
                        )]);
                        table.update(&vec![(
                            [1, 3],
                            format!("\n  {}", country.storage[0].0[2].quantity),
                        )]);
                        table.update(&vec![(
                            [1, 4],
                            format!("\n  {}", country.storage[0].0[3].quantity),
                        )]);


                        self.gui_render.table_up(&false);
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "gui"))]
pub mod gui {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct GuiS {}

    pub type FactsGui = ();
}

//

#[cfg(feature = "window")]
pub mod window {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct WindowS {
        pub(crate) name: String,
        pub(crate) draw: bool,
        pub(crate) aabb: AabbS,

        pub(crate) flooded_border: Option<bool>,
        pub(crate) color: [Color; 2],

        #[cfg(feature = "button")]
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
        fn default() -> &'static WindowS {
            let tmp: WindowS = WindowS::default();
            Box::leak(Box::new(tmp))
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

                #[cfg(feature = "button")]
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

                #[cfg(feature = "button")]
                button: Vec::new(),
            }
        }
    }

    impl Give for WindowS {
        type Output = WindowS;
        type ID = String;

        fn vector_give<'a>(
            facts: &'a Vec<Self::Output>, 
            id: &'a Self::ID
        ) -> Option<&'a Self::Output> {
            facts.iter().find(|data| &data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: &Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| &data.name == id)
        }

        fn hashmap_give<'a>(
            facts: &'a HashMap<Self::ID, Self::Output>,
            id: &'a Self::ID,
            num: bool
        ) -> Option<&'a Self::Output> {
            return if num {
                facts
                    .iter()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: &Self::ID,
            num: bool
        ) -> Option<&'a mut Self::Output> {
            return if num {
                facts
                    .iter_mut()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter_mut()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
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

            if let Some(window) = WindowS::vector_give_mut(vec, name) {
                window.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(window) = WindowS::vector_give_mut(vec, name) {
                window.draw = *meaning;
            }
        }
    }
}

#[cfg(not(feature = "window"))]
pub mod window {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct WindowS {}

    pub type FactsWindow = (());
}

//

#[cfg(feature = "button")]
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
        pub(crate) text: Option<([Color; 2], String)>,
    }

    pub type FactsButton = (
        String,
        bool,
        FactsAabb,
        Option<bool>,
        [Color; 2],
        Option<KeyCode>,
        Vec<ActionE>,
        Option<([Color; 2], String)>,
    );

    impl Default for &ButtonS {
        fn default() -> &'static ButtonS {
            let tmp: ButtonS = ButtonS::default();
            Box::leak(Box::new(tmp))
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

        fn vector_give<'a>(
            facts: &'a Vec<Self::Output>, 
            id: &'a Self::ID
        ) -> Option<&'a Self::Output> {
            facts.iter().find(|data| &data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: &Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| &data.name == id)
        }

        fn hashmap_give<'a>(
            facts: &'a HashMap<Self::ID, Self::Output>,
            id: &'a Self::ID,
            num: bool
        ) -> Option<&'a Self::Output> {
            return if num {
                facts
                    .iter()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: &Self::ID,
            num: bool
        ) -> Option<&'a mut Self::Output> {
            return if num {
                facts
                    .iter_mut()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter_mut()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
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

            if let Some(button) = ButtonS::vector_give_mut(vec, name) {
                button.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(button) = ButtonS::vector_give_mut(vec, name) {
                button.draw = *meaning;
            }
        }
    }

    impl Control for ButtonS {
        type Facts = Option<([Color; 2], String)>;

        fn update(&mut self, facts: &Self::Facts) {
            self.text = facts.clone();
        }
    }
}

#[cfg(not(feature = "button"))]
pub mod button {
    #[derive(Debug, Clone)]
    pub struct ButtonS {}

    pub type FactsButton = (());
}

//

#[cfg(feature = "table")]
pub mod table {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct TableS {
        pub(crate) name: String,
        pub(crate) draw: bool,
        pub(crate) position: PositionS,

        pub(crate) flooded_border: Option<bool>,
        pub(crate) color: [Color; 2],

        pub(crate) indentation: u16,
        pub(crate) cells: Vec<(AabbS, Vec<([Color; 2], String)>)>,
    }

    pub type FactsTable = (
        String,
        bool,
        FactsPosition,
        Option<bool>,
        [Color; 2],
        u16,
        Vec<(FactsAabb, Vec<([Color; 2], String)>)>,
    );

    impl Default for &TableS {
        fn default() -> &'static TableS {
            let tmp: TableS = TableS::default();
            Box::leak(Box::new(tmp))
        }
    }

    impl Create for TableS {
        type Output = TableS;
        type Facts = FactsTable;

        fn new(facts: &Self::Facts) -> Self::Output {
            TableS {
                name: String::from(&facts.0),
                draw: facts.1,
                position: PositionS::new(&facts.2),

                flooded_border: facts.3.clone(),
                color: facts.4,

                indentation: facts.5,
                cells: {
                    let mut vector = Vec::new();

                    for i in &facts.6 {
                        vector.push((AabbS::new(&i.0), i.1.clone(), ));
                    }

                    vector
                },
            }
        }

        fn default() -> Self::Output {
            TableS {
                name: String::new(),
                draw: false,
                position: PositionS::default(),

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

        fn vector_give<'a>(
            facts: &'a Vec<Self::Output>, 
            id: &'a Self::ID
        ) -> Option<&'a Self::Output> {
            facts.iter().find(|data| &data.name == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: &Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| &data.name == id)
        }

        fn hashmap_give<'a>(
            facts: &'a HashMap<Self::ID, Self::Output>,
            id: &'a Self::ID,
            num: bool
        ) -> Option<&'a Self::Output> {
            return if num {
                facts
                    .iter()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: &Self::ID,
            num: bool
        ) -> Option<&'a mut Self::Output> {
            return if num {
                facts
                    .iter_mut()
                    .find(|(_, data)| &data.name == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter_mut()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
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

            if let Some(table) = TableS::vector_give_mut(vec, name) {
                table.draw = *meaning;
            }
        }

        fn open_one_close(vec: &mut Vec<Self::Output>, name: &String, meaning: &bool) {
            if let Some(table) = TableS::vector_give_mut(vec, name) {
                table.draw = *meaning;
            }
        }
    }

    impl Control for TableS {
        type Facts = Vec<([usize; 2], String)>;

        fn update(&mut self, facts: &Self::Facts) {
            for data in facts {
                if let Some(line) = self.cells.get_mut(data.0[0]) {
                    if let Some(cell) = line.1.get_mut(data.0[1]) {
                        cell.1 = data.1.clone();
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "table"))]
pub mod table {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct TableS {}

    pub type FactsTable = (());
}

//

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

        fn vector_give<'a>(
            facts: &'a Vec<Self::Output>, 
            id: &'a Self::ID
        ) -> Option<&'a Self::Output> {
            facts.iter().find(|data| &data.code == id)
        }

        fn vector_give_mut<'a>(
            facts: &'a mut Vec<Self::Output>,
            id: &Self::ID,
        ) -> Option<&'a mut Self::Output> {
            facts.iter_mut().find(|data| &data.code == id)
        }

        fn hashmap_give<'a>(
            facts: &'a HashMap<Self::ID, Self::Output>,
            id: &'a Self::ID,
            num: bool
        ) -> Option<&'a Self::Output> {
            return if num {
                facts
                    .iter()
                    .find(|(_, data)| &data.code == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
        }

        fn hashmap_give_mut<'a>(
            facts: &'a mut HashMap<Self::ID, Self::Output>,
            id: &Self::ID,
            num: bool
        ) -> Option<&'a mut Self::Output> {
            return if num {
                facts
                    .iter_mut()
                    .find(|(_, data)| &data.code == id)
                    .map(|(_, data)| data)
            } else {
                facts
                    .iter_mut()
                    .find(|(data, _)| *data == id)
                    .map(|(_, data)| data)
            }
        }
    }
}

//

pub mod gui_render {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct GuiRenderS {
        pub(crate) gui: bool,
        pub(crate) window: bool,
        pub(crate) button: bool,
        pub(crate) table: bool,
    }

    pub type FactsGuiRender = [bool; 4];

    impl Create for GuiRenderS {
        type Output = GuiRenderS;
        type Facts = FactsGuiRender;

        fn new(facts: &Self::Facts) -> Self::Output {
            GuiRenderS {
                gui: facts[0],
                window: facts[1],
                button: facts[2],
                table: facts[3],
            }
        }

        fn default() -> Self::Output {
            GuiRenderS {
                gui: true,
                window: true,
                button: true,
                table: true,
            }
        }
    }

    impl GuiRenderS {
        pub fn gui_up(&mut self, result: &bool) {
            self.gui = *result;
        }
        pub fn window_up(&mut self, result: &bool) {
            self.window = *result;
        }
        pub fn button_up(&mut self, result: &bool) {
            self.button = *result;
        }
        pub fn table_up(&mut self, result: &bool) {
            self.table = *result;
        }
    }
}