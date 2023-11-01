use crate::*;

#[derive(Debug, Clone)]
pub struct GuiWindowS {
    pub(crate) name: String,
    pub(crate) draw: bool,
    pub(crate) aabb: AabbS,
    pub(crate) texture: TextureT,
    pub(crate) button: Vec<GuiButtonS>,
    pub(crate) text: Vec<GuiTextS>,
}

pub type FactsGuiWindow = (
    String,
    bool,
    FactsAabb,
    TextureT,
    Vec<FactsGuiButton>,
    Vec<(String, FactsAabb)>,
);

impl Create for GuiWindowS {
    type Output = GuiWindowS;
    type Facts = FactsGuiWindow;

    fn new(facts: Self::Facts) -> Self::Output {
        GuiWindowS {
            name: facts.0,
            draw: facts.1,
            aabb: AabbS::new(facts.2),
            texture: facts.3,
            button: GuiButtonS::vec_new(facts.4),
            text: GuiTextS::vec_new(facts.5),
        }
    }

    fn default() -> Self::Output {
        GuiWindowS {
            name: "".to_string(),
            draw: false,
            aabb: AabbS::default(),
            texture: [0.0; 4],
            button: Vec::new(),
            text: Vec::new(),
        }
    }
}
