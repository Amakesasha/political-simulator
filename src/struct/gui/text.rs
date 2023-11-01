use crate::*;

#[derive(Debug, Clone)]
pub struct GuiTextS {
    pub(crate) text: String,
    pub(crate) aabb: AabbS,
}

impl Create for GuiTextS {
    type Output = GuiTextS;
    type Facts = (String, FactsAabb);

    fn new(facts: Self::Facts) -> Self::Output {
        GuiTextS {
            text: facts.0,
            aabb: AabbS::new(facts.1),
        }
    }

    fn default() -> Self::Output {
        GuiTextS {
            text: "".to_string(),
            aabb: AabbS::default(),
        }
    }
}
