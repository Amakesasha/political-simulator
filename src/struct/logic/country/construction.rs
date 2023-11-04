use crate::*;

#[post("/construction/add_factory_tree")]
pub fn add_factory_tree() -> Redirect {
    let mut game = GAME.lock().unwrap();

    if let Some(coutry) = CountryS::give_mut(&mut game.logic.countries, "Country".to_string()) {
        let mut stor = &mut coutry.storage;

        if stor.concrete.quantity >= stor.wood.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.wood.resources_required_for_construction[1]
            && stor.wood.quantity >= stor.wood.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.wood.resources_required_for_construction[2];
            stor.iron.quantity -= stor.wood.resources_required_for_construction[1];
            stor.wood.quantity -= stor.wood.resources_required_for_construction[0];

            stor.wood.number_of_factories += 1;
        }
    }

    Redirect::to("/game")
}

#[post("/construction/add_factory_iron")]
pub fn add_factory_iron() -> Redirect {
    let mut game = GAME.lock().unwrap();

    if let Some(coutry) = CountryS::give_mut(&mut game.logic.countries, "Country".to_string()) {
        let mut stor = &mut coutry.storage;

        if stor.concrete.quantity >= stor.iron.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.iron.resources_required_for_construction[1]
            && stor.wood.quantity >= stor.iron.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.iron.resources_required_for_construction[2];
            stor.iron.quantity -= stor.iron.resources_required_for_construction[1];
            stor.wood.quantity -= stor.iron.resources_required_for_construction[0];

            stor.iron.number_of_factories += 1;
        }
    }

    Redirect::to("/game")
}

#[post("/construction/add_factory_concrete")]
pub fn add_factory_concrete() -> Redirect {
    let mut game = GAME.lock().unwrap();

    if let Some(coutry) = CountryS::give_mut(&mut game.logic.countries, "Country".to_string()) {
        let mut stor = &mut coutry.storage;

        if stor.concrete.quantity >= stor.concrete.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.concrete.resources_required_for_construction[1]
            && stor.wood.quantity >= stor.concrete.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.concrete.resources_required_for_construction[2];
            stor.iron.quantity -= stor.concrete.resources_required_for_construction[1];
            stor.wood.quantity -= stor.concrete.resources_required_for_construction[0];

            stor.concrete.number_of_factories += 1;
        }
    }

    Redirect::to("/game")
}
