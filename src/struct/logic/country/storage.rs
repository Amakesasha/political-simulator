use crate::*;

#[derive(Debug, Clone)]
pub struct StorageS {
    pub(crate) concrete: ResourceS,
    pub(crate) wood: ResourceS,
    pub(crate) iron: ResourceS,
}

pub type FactsStorage = [FactsResource; 3];

impl Create for StorageS {
    type Output = StorageS;
    type Facts = FactsStorage;

    fn new(facts: Self::Facts) -> Self::Output {
        StorageS {
            concrete: ResourceS::new(facts[0]),
            wood: ResourceS::new(facts[1]),
            iron: ResourceS::new(facts[2]),
        }
    }

    fn default() -> Self::Output {
        StorageS {
            concrete: ResourceS::default(),
            wood: ResourceS::default(),
            iron: ResourceS::default(),
        }
    }
}

impl Control for StorageS {
    type Facts = [usize; 3];

    fn update(&mut self, facts: Self::Facts) {
        self.concrete.update(facts);
        self.wood.update(facts);
        self.iron.update(facts);
    }
}

pub mod resource {
    use crate::*;

    #[derive(Debug, Clone)]
    pub struct ResourceS {
        pub(crate) quantity: f64,
        pub(crate) price: f64,

        pub(crate) number_of_factories: usize,
        pub(crate) production_1_plant: f64,
        pub(crate) resources_required_for_construction: [f64; 3],
    }

    pub type FactsResource = (f64, f64, usize, f64, [f64; 3]);

    impl Create for ResourceS {
        type Output = ResourceS;
        type Facts = FactsResource;

        fn new(facts: Self::Facts) -> Self::Output {
            ResourceS {
                quantity: facts.0,
                price: facts.1,

                number_of_factories: facts.2,
                production_1_plant: facts.3,
                resources_required_for_construction: facts.4,
            }
        }

        fn default() -> Self::Output {
            ResourceS {
                quantity: 0.0,
                price: 0.0,

                number_of_factories: 0,
                production_1_plant: 0.0,
                resources_required_for_construction: [0.0; 3],
            }
        }
    }

    impl Control for ResourceS {
        type Facts = [usize; 3];

        fn update(&mut self, facts: Self::Facts) {
            let factor = (facts[0] + (facts[1] * 30) + (facts[2] * 360)) as f64;

            let income = self.number_of_factories as f64 * self.production_1_plant * factor;

            self.quantity += income;
        }
    }
}
