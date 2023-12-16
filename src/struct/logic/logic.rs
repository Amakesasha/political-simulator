use crate::*;

#[derive(Debug, Clone)]
pub struct LogicS {
    pub(crate) date: DateS,
    pub(crate) name_country: [String; 2],

    pub(crate) countries: HashMap<String, CountryS>,

    pub(crate) resources_for_construction: [[[f64; 4]; 4]; 2],
}

pub type FactsLogic = (
    FactsDate,
    [String; 2],
    Vec<String>,
    Vec<FactsCountry>,
    [[[f64; 4]; 4]; 2],
);

impl Create for LogicS {
    type Output = LogicS;
    type Facts = FactsLogic;

    fn new(facts: &Self::Facts) -> Self::Output {
        LogicS {
            date: DateS::new(&facts.0),
            name_country: facts.1.clone(),

            countries: CountryS::hash_map_new(&facts.2, &facts.3),

            resources_for_construction: facts.4,
        }
    }

    fn default() -> Self::Output {
        LogicS {
            date: DateS::default(),
            name_country: [String::new(), String::new()],

            countries: HashMap::new(),

            resources_for_construction: [[[0.0; 4]; 4]; 2],
        }
    }
}

impl Control for LogicS {
    type Facts = ();

    fn update(&mut self, _facts: &Self::Facts) {
        self.date.update(&());

        for (_, country) in &mut self.countries {
            country.update(&self.date);
        }
    }
}
