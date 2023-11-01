use crate::*;

#[derive(Debug, Clone)]
pub struct GuiS {
    pub(crate) button: Vec<GuiButtonS>,
    pub(crate) window: Vec<GuiWindowS>,
}

pub type TextureT = [f32; 4];

pub type FactsGui = (Vec<FactsGuiButton>, Vec<FactsGuiWindow>);

impl Create for GuiS {
    type Output = GuiS;
    type Facts = FactsGui;

    fn new(facts: Self::Facts) -> Self::Output {
        GuiS {
            button: GuiButtonS::vec_new(facts.0),
            window: GuiWindowS::vec_new(facts.1),
        }
    }

    fn default() -> Self::Output {
        GuiS {
            button: Vec::new(),
            window: Vec::new(),
        }
    }
}
