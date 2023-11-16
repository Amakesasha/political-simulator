use crate::*;

/*
    Resource Id:
        0 => Concrete;
        1 => Wood;
        2 => Iron;
        3 => Rubber;
*/

#[derive(Debug, Clone, Copy)]
pub struct StorageS (pub(crate) [ResourceS; 4]);

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
                ResourceS::new(&facts[3])
            ]
        }
    }

    fn default() -> Self::Output {
        StorageS {
            0: [ResourceS::default(); 4]
        }
    }
}

impl Control for StorageS {
    type Facts = [usize; 3];

    fn update(&mut self, facts: Self::Facts) {
        for resource in &mut self.0 {
            resource.update(facts);
        }
    }
}

pub mod resource {
    use crate::*;

    #[derive(Debug, Clone, Copy)]
    pub struct ResourceS {
        pub(crate) quantity: f64,
        pub(crate) price: f64,

        pub(crate) number_of_factories: usize,
        pub(crate) production_1_plant: f64,
    }

    pub type FactsResource = (f64, f64, usize, f64);

    impl Create for ResourceS {
        type Output = ResourceS;
        type Facts = FactsResource;

        fn new(facts: &Self::Facts) -> Self::Output {
            ResourceS {
                quantity: facts.0,
                price: facts.1,

                number_of_factories: facts.2,
                production_1_plant: facts.3,
            }
        }

        fn default() -> Self::Output {
            ResourceS {
                quantity: 0.0,
                price: 0.0,

                number_of_factories: 0,
                production_1_plant: 0.0,
            }
        }
    }

    impl Control for ResourceS {
        type Facts = [usize; 3];

        fn update(&mut self, facts: Self::Facts) {
            let factor = (facts[0] + (facts[1] * 30) + (facts[2] * 360)) as f64;

            let income = self.number_of_factories as f64 * self.production_1_plant * factor;

            self.quantity += income;
        }
    }

    pub mod web {
        use crate::*;

        #[put("/game/logic/resourse/update_color_button_build", data = "<facts>")]
        pub fn update_color_button_build(facts: Form<Message>) -> String {
            let game = GAME.lock().unwrap();

            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let stor = &country.storage.0;

                let num = facts.0.a.trim().parse::<usize>().unwrap_or(0);

                if stor[0].quantity >= game.logic.resources_for_construction_factory[num][0]
                    && stor[1].quantity >= game.logic.resources_for_construction_factory[num][1]
                    && stor[2].quantity >= game.logic.resources_for_construction_factory[num][2]
                {
                    format!("#008000")
                } else {
                    format!("#FF0000")
                }
            } else {
                format!("#FF0000")
            }
        }

        #[put("/game/logic/resourse/update_color_button_destroy", data = "<facts>")]
        pub fn update_color_button_destroy(facts: Form<Message>) -> String {
            let game = GAME.lock().unwrap();

            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let stor = &country.storage.0;

                let num = facts.0.a.trim().parse::<usize>().unwrap_or(0);

                if stor[num].number_of_factories > 0 {
                    format!("#008000")
                } else {
                    format!("#FF0000")
                }
            } else {
                format!("#FF0000")
            }
        }

        #[put("/game/logic/resourse/update_quantity_resourse", data = "<facts>")]
        pub fn update_quantity_resourse(facts: Form<Message>) -> String {
            let game = GAME.lock().unwrap();

            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let stor = &country.storage.0;

                let num = facts.0.a.trim().parse::<usize>().unwrap_or(0);

                format!("{}", stor[num].quantity)
            } else {
                format!("0")
            }
        }
    }
}