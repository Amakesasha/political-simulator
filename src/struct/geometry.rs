use crate::*;

pub mod aabb {
	use crate::*;

	#[derive(Debug, Clone)]
	pub struct AabbS {
		pub(crate) position: f64,
		pub(crate) size: Scale,
	}

	impl Create for AabbS {
		type Output = AabbS;
		type Facts = [[f64; 2]; 2];

		fn new(facts: Self::Output) -> Self::Output {
			AabbS {
				position: PositionS::new(facts[0]),
				size: Scale::new(facts[1]),
			}
		}

		fn default() -> Self::Output {
			AabbS {
				position: PositionS::default(),
				size: Scale::default(),
			}
		}
	}
}


pub mod position {
	use crate::*;

	#[derive(Debug, Clone)]
	pub struct PositionS {
		pub(crate) x: f64,
		pub(crate) y: f64,
	}

	impl Create for PositionS {
		type Output = PositionS;
		type Facts = [f64; 2];

		fn new(facts: Self::Output) -> Self::Output {
			PositionS {
				x: facts.0,
				y: facts.1,
			}
		}

		fn default() -> Self::Output {
			PositionS {
				x: 0.0,
				y: 0.0,
			}
		}
	}
}

pub mod size {
	use crate::*;

	#[derive(Debug, Clone)]
	pub struct Scale {
		pub(crate) width: f64,
		pub(crate) height: f64,
	}

	impl Create for Scale {
		type Output = Scale;
		type Facts = [f64; 2];

		fn new(facts: Self::Output) -> Self::Output {
			Scale {
				width: facts.0,
				height: facts.1,
			}
		}

		fn default() -> Self::Output {
			Scale {
				width: 0.0,
				height: 0.0,
			}
		}
	}
}