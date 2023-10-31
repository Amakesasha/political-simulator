<<<<<<< HEAD
use crate::*;

#[derive(Debug, Clone)]
pub struct GuiButtonS {
	pub(crate) name: String,
	pub(crate) aabb: AabbS,
	pub(crate) texture: TextureT,
}

impl Create for GuiButtonS {
	type Output = GuiButtonS;
	type Facts = (Stringl, [[f64; 2]; 2], TextureT);

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
=======
use crate::*;

#[derive(Debug, Clone)]
pub struct GuiButtonS {
	pub(crate) name: String,
	pub(crate) position: AabbS,
>>>>>>> dfed9cbafd8402bed0a415b7b580a4fbb0959c13
}