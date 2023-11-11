pub mod build {
    use crate::*;

    #[post("/logic/construction/build/build_factory_concrete")]
    pub fn build_factory_concrete() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.concrete.quantity >= stor.concrete.resources_required_for_construction[0]
                && stor.wood.quantity >= stor.concrete.resources_required_for_construction[1]
                && stor.iron.quantity >= stor.concrete.resources_required_for_construction[2]
            {
                stor.concrete.quantity -= stor.concrete.resources_required_for_construction[0];
                stor.wood.quantity -= stor.concrete.resources_required_for_construction[1];
                stor.iron.quantity -= stor.concrete.resources_required_for_construction[2];

                stor.concrete.number_of_factories += 1;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }

    #[post("/logic/construction/build/build_factory_wood")]
    pub fn build_factory_wood() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.concrete.quantity >= stor.wood.resources_required_for_construction[0]
                && stor.wood.quantity >= stor.wood.resources_required_for_construction[1]
                && stor.iron.quantity >= stor.wood.resources_required_for_construction[2]
            {
                stor.concrete.quantity -= stor.wood.resources_required_for_construction[0];
                stor.wood.quantity -= stor.wood.resources_required_for_construction[1];
                stor.iron.quantity -= stor.wood.resources_required_for_construction[2];

                stor.wood.number_of_factories += 1;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }

    #[post("/logic/construction/build/build_factory_iron")]
    pub fn build_factory_iron() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.concrete.quantity >= stor.iron.resources_required_for_construction[0]
                && stor.wood.quantity >= stor.iron.resources_required_for_construction[1]
                && stor.iron.quantity >= stor.iron.resources_required_for_construction[2]
            {
                stor.concrete.quantity -= stor.iron.resources_required_for_construction[0];
                stor.wood.quantity -= stor.iron.resources_required_for_construction[1];
                stor.iron.quantity -= stor.iron.resources_required_for_construction[2];

                stor.iron.number_of_factories += 1;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }
}

pub mod destroy {
    use crate::*;

    #[post("/logic/construction/destroy/destroy_factory_concrete")]
    pub fn destroy_factory_concrete() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.concrete.number_of_factories > 0 {
                stor.concrete.number_of_factories -= 1;

                stor.concrete.quantity +=
                    stor.concrete.resources_required_for_construction[0] * 0.75;
                stor.wood.quantity += stor.concrete.resources_required_for_construction[1] * 0.75;
                stor.iron.quantity += stor.concrete.resources_required_for_construction[2] * 0.75;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }

    #[post("/logic/construction/destroy/destroy_factory_wood")]
    pub fn destroy_factory_wood() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.wood.number_of_factories > 0 {
                stor.wood.number_of_factories -= 1;

                stor.concrete.quantity += stor.wood.resources_required_for_construction[0] * 0.75;
                stor.wood.quantity += stor.wood.resources_required_for_construction[1] * 0.75;
                stor.iron.quantity += stor.wood.resources_required_for_construction[2] * 0.75;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }

    #[post("/logic/construction/destroy/destroy_factory_iron")]
    pub fn destroy_factory_iron() -> Redirect {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let mut stor = &mut country.storage;

            if stor.iron.number_of_factories > 0 {
                stor.iron.number_of_factories -= 1;

                stor.concrete.quantity += stor.iron.resources_required_for_construction[0] * 0.75;
                stor.wood.quantity += stor.iron.resources_required_for_construction[1] * 0.75;
                stor.iron.quantity += stor.iron.resources_required_for_construction[2] * 0.75;
            }

            Redirect::to("/game/resourse")
        } else {
            Redirect::to("/login/window")
        }
    }
}
