use crate::*;

#[derive(Debug, Clone)]
pub struct GuiButtonS {
	pub(crate) name: String,
	pub(crate) aabb: AabbS,
	pub(crate) texture: TextureT,
}

pub type FactsGuiButton = (String, FactsAabb, TextureT);

impl Create for GuiButtonS {
	type Output = GuiButtonS;
	type Facts = FactsGuiButton;

	fn new(facts: Self::Facts) -> Self::Output {
		GuiButtonS {
			name: facts.0,
			aabb: AabbS::new(facts.1),
			texture: facts.2,
		}
	}

	fn default() -> Self::Output {
		GuiButtonS {
			name: "".to_string(),
			aabb: AabbS::default(),
			texture: Err(None),
		}
	}
}