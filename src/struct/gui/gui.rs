<<<<<<< HEAD
use crate::*;

#[derive(Debug, Clone)]
pub struct GuiS {
	pub(crate) button: Vec<GuiButtonS>,
	pub(crate) window: Vec<GuiWindowS>,
}

pub type TextureT = Result<
    Result<Vec<Texture<gfx_device_gl::Resources>>, Texture<gfx_device_gl::Resources>>,
    Option<[f32; 4]>,
>;


impl Create for GuiS {
	type Output = GuiS;
	type Facts = (DataGuiButton, DataGuiWindow);

	fn new(facts: Self::Facts) -> Self::Output {
		GuiS {
			button: GuiButtonS::vec_new(facts.0),
			window: GuiWindowS::vec_new(facts.1),
		}
	}

	fn default() -> Self::Output {
		GuiS {
			button: Vec::new(),
			window:Vec::new(),
		}
	}
=======
use crate::*;

#[derive(Debug, Clone)]
pub struct GuiS {
	pub(crate) button: Vec<GuiButtonS>,
	pub(crate) window: Vec<GuiWindowS>,
}

pub type TextureT = Result<
    Result<Vec<Texture<gfx_device_gl::Resources>>, Texture<gfx_device_gl::Resources>>,
    Option<[f32; 4]>,
>;


impl Create for GuiS {
	type Output = GuiS;
	type Facts = (DataGuiButton, DataGuiWindow);

	fn new(facts: Self::Facts) -> Self::Output {
		GuiS {
			button: GuiButtonS::vec_new(facts.0),
			window: GuiWindowS::vec_new(facts.1),
		}
	}

	fn default() -> Self::Output {
		GuiS {
			button: Vec::new(),
			window:Vec::new(),
		}
	}
>>>>>>> dfed9cbafd8402bed0a415b7b580a4fbb0959c13
}