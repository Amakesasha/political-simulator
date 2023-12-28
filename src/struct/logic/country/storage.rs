use crate::*;

/*
    Resource Id:
        0 => Concrete;
        1 => Wood;
        2 => Iron;
        3 => Rubber;
*/

pub mod storage {
    use crate::*;

    pub const NUM_RES: usize = 4;
    pub const NUM_STOR: usize = 2;

    #[derive(Debug, Clone, Copy)]
    pub struct StorageS(pub(crate) [ResourceS; NUM_RES]);

    pub type FactsStorage = [FactsResource; NUM_RES];

    impl Create for StorageS {
        type Output = StorageS;
        type Facts = FactsStorage;

        fn new(facts: &Self::Facts) -> Self::Output {
            StorageS {
                0: [
                    ResourceS::new(&facts[0]),
                    ResourceS::new(&facts[1]),
                    ResourceS::new(&facts[2]),
                    ResourceS::new(&facts[3]),
                ],
            }
        }

        fn default() -> Self::Output {
            StorageS {
                0: [ResourceS::default(); NUM_RES],
            }
        }

        fn default_facts() -> Self::Facts {
            [ResourceS::default_facts(); NUM_RES]
        }
    }

    impl Control for StorageS {
        type Facts = DateS;

        fn update(&mut self, facts: &Self::Facts) {
            for resource in &mut self.0 {
                resource.update(&facts);
            }
        }
    }
}

//

pub mod resource {
    use crate::*;

    #[derive(Debug, Clone, Copy)]
    pub struct ResourceS {
        pub(crate) quantity: f64,

        pub(crate) storage: usize,
        pub(crate) capacity_storage: f64,

        pub(crate) number_of_factory: usize,
        pub(crate) production_1_factory: f64,
    }

    pub type FactsResource = (f64, usize, f64, usize, f64);

    impl Create for ResourceS {
        type Output = ResourceS;
        type Facts = FactsResource;

        fn new(facts: &Self::Facts) -> Self::Output {
            ResourceS {
                quantity: facts.0,

                storage: facts.1,
                capacity_storage: facts.2,

                number_of_factory: facts.3,
                production_1_factory: facts.4,
            }
        }

        fn default() -> Self::Output {
            ResourceS {
                quantity: 0.0,

                storage: 0,
                capacity_storage: 0.0,

                number_of_factory: 0,
                production_1_factory: 0.0,
            }
        }

        fn default_facts() -> Self::Facts {
            (0.0, 0, 0.0, 0, 0.0)
        }
    }

    impl Control for ResourceS {
        type Facts = DateS;

        fn update(&mut self, facts: &Self::Facts) {
            let factor =
                (facts.update[0] + (facts.update[1] * 30) + (facts.update[2] * 360)) as f64;

            let income = self.number_of_factory as f64 * self.production_1_factory * factor;

            if self.quantity + income <= self.storage as f64 * self.capacity_storage {
                self.quantity += income;
            } else {
                self.quantity = self.storage as f64 * self.capacity_storage;
            }
        }
    }
}
