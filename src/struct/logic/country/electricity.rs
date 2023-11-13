use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct ElectricityS ( pub(crate) [PowerStationS; 4] );

pub type FactsElectricity = [FactsPowerStation; 4];

impl Create for ElectricityS {
	type Output = ElectricityS;
	type Facts = FactsElectricity;

	fn new(facts: &Self::Facts) -> Self::Output {
		ElectricityS {
			0: [
				PowerStationS::new(&facts[0]),
				PowerStationS::new(&facts[1]),
				PowerStationS::new(&facts[2]),
				PowerStationS::new(&facts[3]),
			]
		}
	}

	fn default() -> Self::Output {
		ElectricityS {
			0: [PowerStationS::default(); 4]
		}
	}
}

pub mod power_station {
	use crate::*;

	#[derive(Debug, Clone, Copy)]
	pub struct PowerStationS {
        pub(crate) quantity: usize,
        pub(crate) production_1_plant: f64,
	}

	pub type FactsPowerStation = (usize, f64);

	impl Create for PowerStationS {
		type Output = PowerStationS;
		type Facts = FactsPowerStation;

		fn new(facts: &Self::Facts) -> Self::Output {
			PowerStationS {
				quantity: facts.0,
				production_1_plant: facts.1,
			}
		}

		fn default() -> Self::Output {
			PowerStationS {
				quantity: 0,
				production_1_plant: 0.0,
			}
		}
	}
}