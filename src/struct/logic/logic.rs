use crate::*;

#[derive(Debug, Clone)]
pub struct LogicS {
    pub(crate) date: DateS,
    pub(crate) countries: HashMap<String, CountryS>,
    pub(crate) name_country: String,
}

pub type FactsLogic = (FactsDate, Vec<String>, Vec<FactsCountry>, String);

impl Create for LogicS {
    type Output = LogicS;
    type Facts = FactsLogic;

    fn new(facts: &Self::Facts) -> Self::Output {
        LogicS {
            date: DateS::new(&facts.0),
            countries: CountryS::hash_map_new(&facts.1, &facts.2),
            name_country: facts.3.clone(),
        }
    }

    fn default() -> Self::Output {
        LogicS {
            date: DateS::default(),
            countries: HashMap::new(),
            name_country: String::new(),
        }
    }
}

impl Control for LogicS {
    type Facts = ();

    fn update(&mut self, _facts: Self::Facts) {
        self.date.update(());

        for country in &mut self.countries {
            country.1.update(self.date.update);
        }
    }
}
