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

    fn new(facts: Self::Facts) -> Self::Output {
        CountryS {
            name: facts.0,
            storage: StorageS::new(facts.1),
        }
    }

    fn default() -> Self::Output {
        CountryS {
            name: "".to_string(),
            storage: StorageS::default(),
        }
    }
}

impl Give for CountryS {
    type Output = CountryS;
    type ID = String;

    fn give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<Self::Output> {
        return if let Some(index) = facts.iter().position(|country| country.name == id) {
            Some(facts[index].clone())
        } else {
            None
        };
    }

    fn give_mut<'a>(
        facts: &'a mut Vec<Self::Output>,
        id: Self::ID,
    ) -> Option<&'a mut Self::Output> {
        if let Some(index) = facts.iter().position(|country| country.name == id) {
            Some(&mut facts[index])
        } else {
            None
        }
    }
}

impl Control for CountryS {
    type Facts = [usize; 3];

    fn update(&mut self, facts: Self::Facts) {
        self.storage.update(facts);
    }
}
