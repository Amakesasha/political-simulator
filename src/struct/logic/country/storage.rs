use crate::*;

/*
    Resource Id:
        0 => Concrete;
        1 => Wood;
        2 => Iron;
        3 => Rubber;
*/

#[derive(Debug, Clone, Copy)]
pub struct StorageS(pub(crate) [ResourceS; 4]);

pub type FactsStorage = [FactsResource; 4];

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
            0: [ResourceS::default(); 4],
        }
    }
}

impl Control for StorageS {
    type Facts = DateS;

    fn update(&mut self, facts: Self::Facts) {
        for resource in &mut self.0 {
            resource.update(facts);
        }
    }
}

//

#[derive(Debug, Clone, Copy)]
pub struct ResourceS {
    pub(crate) quantity: f64,
    pub(crate) price: f64,

    pub(crate) number_of_factory: usize,
    pub(crate) production_1_factory: f64,
}

pub type FactsResource = (f64, f64, usize, f64);

impl Create for ResourceS {
    type Output = ResourceS;
    type Facts = FactsResource;

    fn new(facts: &Self::Facts) -> Self::Output {
        ResourceS {
            quantity: facts.0,
            price: facts.1,

            number_of_factory: facts.2,
            production_1_factory: facts.3,
        }
    }

    fn default() -> Self::Output {
        ResourceS {
            quantity: 0.0,
            price: 0.0,

            number_of_factory: 0,
            production_1_factory: 0.0,
        }
    }
}

impl Control for ResourceS {
    type Facts = DateS;

    fn update(&mut self, facts: Self::Facts) {
        let factor = (facts.date[0] + (facts.date[1] * 30) + (facts.date[2] * 360)) as f64;

        let income = self.number_of_factory as f64 * self.production_1_factory * factor;

        self.quantity += income;
    }
}
