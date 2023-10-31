use crate::*;

#[derive(Debug, Clone)]
pub struct GuiWindowS {
	pub(crate) name: String,
	pub(crate) aabb: AabbS,
	pub(crate) texture: TextureT,
	pub(crate) button: Vec<GuiButtonS>
}

pub type FactsGuiWindow = (String, FactsAabb, TextureT, Vec<FactsGuiButton>);

impl Create for GuiWindowS {
	type Output = GuiWindowS;
	type Facts = FactsGuiWindow;

	fn new(facts: Self::Facts) -> Self::Output {
		GuiWindowS {
			name: facts.0,
			aabb: AabbS::new(facts.1),
			texture: facts.2,
			button: GuiButtonS::vec_new(facts.3),
		}
	}

	fn default() -> Self::Output {
		GuiWindowS {
			name: "".to_string(),
			aabb: AabbS::default(),
			texture: Err(None),
			button: Vec::new(),
		}
	}
}