use crate::*;

#[derive(Debug, Clone)]
pub struct CountryS {
    pub(crate) name: String,
    pub(crate) storage: StorageS,
}

pub type FactsCountry = (String, FactsStorage);

impl Create for CountryS {
    type Output = CountryS;
    type Facts = FactsCountry;

    fn new(facts: &Self::Facts) -> Self::Output {
        CountryS {
            name: facts.0.clone(),
            storage: StorageS::new(&facts.1),
        }
    }

    fn default() -> Self::Output {
        CountryS {
            name: String::new(),
            storage: StorageS::default(),
        }
    }
}

impl Control for CountryS {
    type Facts = [usize; 3];

    fn update(&mut self, facts: Self::Facts) {
        self.storage.update(facts);
    }
}
