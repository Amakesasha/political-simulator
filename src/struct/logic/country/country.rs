use crate::*;

#[derive(Debug, Clone)]
pub struct CountryS {
    pub(crate) name: String,
    pub(crate) storage: [StorageS; NUM_STOR],
}

pub type FactsCountry = (String, [FactsStorage; NUM_STOR]);

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
            storage: [StorageS::default(); NUM_STOR],
        }
    }

    fn default_facts() -> Self::Facts {
        (String::new(), [StorageS::default_facts(); NUM_STOR])
    }
}

impl Give for CountryS {
    type Output = CountryS;
    type ID = String;

    fn vector_give<'a>(facts: &'a Vec<Self::Output>, id: &'a Self::ID) -> Option<&'a Self::Output> {
        facts.iter().find(|data| &data.name == id)
    }

    fn vector_give_mut<'a>(
        facts: &'a mut Vec<Self::Output>,
        id: &Self::ID,
    ) -> Option<&'a mut Self::Output> {
        facts.iter_mut().find(|data| &data.name == id)
    }

    fn hashmap_give<'a>(
        facts: &'a HashMap<Self::ID, Self::Output>,
        id: &'a Self::ID,
        num: bool,
    ) -> Option<&'a Self::Output> {
        return if num {
            facts
                .iter()
                .find(|(_, data)| &data.name == id)
                .map(|(_, data)| data)
        } else {
            facts
                .iter()
                .find(|(data, _)| *data == id)
                .map(|(_, data)| data)
        };
    }

    fn hashmap_give_mut<'a>(
        facts: &'a mut HashMap<Self::ID, Self::Output>,
        id: &Self::ID,
        num: bool,
    ) -> Option<&'a mut Self::Output> {
        return if num {
            facts
                .iter_mut()
                .find(|(_, data)| &data.name == id)
                .map(|(_, data)| data)
        } else {
            facts
                .iter_mut()
                .find(|(data, _)| *data == id)
                .map(|(_, data)| data)
        };
    }
}

impl Control for CountryS {
    type Facts = DateS;

    fn update(&mut self, facts: &Self::Facts) {
        self.storage[0].update(&facts);
        self.storage[1].update(&facts);
    }
}
