use crate::*;

#[derive(Debug, Clone)]
pub struct CountryS {
    pub(crate) name: String,
    pub(crate) storage: StorageS,
    pub(crate) electricity: ElectricityS,
}

pub type FactsCountry = (String, FactsStorage, FactsElectricity);

impl Create for CountryS {
    type Output = CountryS;
    type Facts = FactsCountry;

    fn new(facts: &Self::Facts) -> Self::Output {
        CountryS {
            name: facts.0.clone(),
            storage: StorageS::new(&facts.1),
            electricity: ElectricityS::new(&facts.2),
        }
    }

    fn default() -> Self::Output {
        CountryS {
            name: String::new(),
            storage: StorageS::default(),
            electricity: ElectricityS::default(),
        }
    }
}

impl Control for CountryS {
    type Facts = [usize; 3];

    fn update(&mut self, facts: Self::Facts) {
        self.storage.update(facts);
    }
}
