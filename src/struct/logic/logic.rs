use crate::*;

#[derive(Debug, Clone)]
pub struct LogicS {
    pub(crate) date: DateS,
    pub(crate) name_country: String,

    pub(crate) countries: HashMap<String, CountryS>,

    pub(crate) resources_for_construction_factory: [[f64; 4]; 4],
    pub(crate) resources_for_construction_power_stantion: [[f64; 4]; 4],
}

pub type FactsLogic = (FactsDate, String, Vec<String>, Vec<FactsCountry>, [[f64; 4]; 4], [[f64; 4]; 4]);

impl Create for LogicS {
    type Output = LogicS;
    type Facts = FactsLogic;

    fn new(facts: &Self::Facts) -> Self::Output {
        LogicS {
            date: DateS::new(&facts.0),
            name_country: facts.1.clone(),

            countries: CountryS::hash_map_new(&facts.2, &facts.3),

            resources_for_construction_factory: facts.4,
            resources_for_construction_power_stantion: facts.5,
        }
    }

    fn default() -> Self::Output {
        LogicS {
            date: DateS::default(),
            name_country: String::new(),

            countries: HashMap::new(),
            
            resources_for_construction_factory: [[0.0; 4]; 4],
            resources_for_construction_power_stantion: [[0.0; 4]; 4]
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
