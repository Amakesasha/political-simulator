use crate::*;

#[derive(Debug, Clone)]
pub struct CountryS {
    pub(crate) name: String,
    pub(crate) storage: [StorageS; 2],
}

pub type FactsCountry = (String, [FactsStorage; 2]);

impl Create for CountryS {
    type Output = CountryS;
    type Facts = FactsCountry;

    fn new(facts: &Self::Facts) -> Self::Output {
        CountryS {
            name: facts.0.clone(),
            storage: [StorageS::new(&facts.1[0]), StorageS::new(&facts.1[1])],
        }
    }

    fn default() -> Self::Output {
        CountryS {
            name: String::new(),
            storage: [StorageS::default(), StorageS::default()],
        }
    }
}

impl Give for CountryS {
    type Output = CountryS;
    type ID = String;

    fn vector_give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<&Self::Output> {
        facts.iter().find(|data| data.name == id)
    }

    fn vector_give_mut<'a>(
        facts: &'a mut Vec<Self::Output>,
        id: Self::ID,
    ) -> Option<&'a mut Self::Output> {
        facts.iter_mut().find(|data| data.name == id)
    }

    fn hashmap_give(
        facts: &HashMap<Self::ID, Self::Output>,
        id: Self::ID,
    ) -> Option<&Self::Output> {
        facts
            .iter()
            .find(|(_, data)| data.name == id)
            .map(|(_, data)| data)
    }

    fn hashmap_give_mut<'a>(
        facts: &'a mut HashMap<Self::ID, Self::Output>,
        id: Self::ID,
    ) -> Option<&'a mut Self::Output> {
        facts
            .iter_mut()
            .find(|(_, data)| data.name == id)
            .map(|(_, data)| data)
    }
}

impl Control for CountryS {
    type Facts = DateS;

    fn update(&mut self, facts: &Self::Facts) {
        self.storage[0].update(&facts);
        self.storage[1].update(&facts);
    }
}
