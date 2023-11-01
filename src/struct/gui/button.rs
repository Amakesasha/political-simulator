use crate::*;

#[derive(Debug, Clone)]
pub struct GuiButtonS {
    pub(crate) name: String,
    pub(crate) draw: bool,
    pub(crate) aabb: AabbS,
    pub(crate) texture: TextureT,
    pub(crate) text: Vec<GuiTextS>,
}

pub type FactsGuiButton = (String, bool, FactsAabb, TextureT, Vec<(String, FactsAabb)>);

impl Create for GuiButtonS {
    type Output = GuiButtonS;
    type Facts = FactsGuiButton;

    fn new(facts: Self::Facts) -> Self::Output {
        GuiButtonS {
            name: facts.0,
            draw: facts.1,
            aabb: AabbS::new(facts.2),
            texture: facts.3,
            text: GuiTextS::vec_new(facts.4),
        }
    }

    fn default() -> Self::Output {
        GuiButtonS {
            name: "".to_string(),
            draw: false,
            aabb: AabbS::default(),
            texture: [0.0; 4],
            text: Vec::new(),
        }
    }
}
