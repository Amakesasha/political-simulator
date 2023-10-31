use crate::*;

#[derive(Debug, Clone)]
pub struct GuiButtonS {
	pub(crate) name: String,
	pub(crate) position: AabbS,
	pub(crate) texture: TextureT,
}

impl Create for GuiButtonS {
	type Output = GuiButtonS;
	type Facts = ();
}