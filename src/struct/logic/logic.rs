use crate::*;

#[derive(Debug, Clone)]
pub struct LogicS {
    pub(crate) date: DateS,
    pub(crate) countries: Vec<CountryS>,
}

pub type FactsLogic = (FactsDate, Vec<FactsCountry>);

impl Create for LogicS {
    type Output = LogicS;
    type Facts = FactsLogic;

    fn new(facts: Self::Facts) -> Self::Output {
        LogicS {
            date: DateS::new(facts.0),
            countries: CountryS::vec_new(facts.1),
        }
    }

    fn default() -> Self::Output {
        LogicS {
            date: DateS::default(),
            countries: Vec::new(),
        }
    }
}

impl Control for LogicS {
    type Facts = ();

    fn update(&mut self, _facts: Self::Facts) {
        self.date.update(());

        for country in &mut self.countries {
            country.update(self.date.update);
        }
    }
}
