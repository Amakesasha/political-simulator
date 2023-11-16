pub mod build {
    use crate::*;

    #[put("/game/logic/construction/build/build_factory_concrete")]
    pub fn build_factory_concrete() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[0].quantity >= resources_for_construction_factory[0][0]
                && stor[1].quantity >= resources_for_construction_factory[0][1]
                && stor[2].quantity >= resources_for_construction_factory[0][2]
            {
                stor[0].quantity -= resources_for_construction_factory[0][0];
                stor[1].quantity -= resources_for_construction_factory[0][1];
                stor[2].quantity -= resources_for_construction_factory[0][2];

                stor[0].number_of_factories += 1;
            }

            format!("{}", stor[0].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/build/build_factory_wood")]
    pub fn build_factory_wood() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[0].quantity >= resources_for_construction_factory[1][0]
                && stor[1].quantity >= resources_for_construction_factory[1][1]
                && stor[2].quantity >= resources_for_construction_factory[1][2]
            {
                stor[0].quantity -= resources_for_construction_factory[1][0];
                stor[1].quantity -= resources_for_construction_factory[1][1];
                stor[2].quantity -= resources_for_construction_factory[1][2];

                stor[1].number_of_factories += 1;
            }

            format!("{}", stor[1].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/build/build_factory_iron")]
    pub fn build_factory_iron() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[0].quantity >= resources_for_construction_factory[2][0]
                && stor[1].quantity >= resources_for_construction_factory[2][1]
                && stor[2].quantity >= resources_for_construction_factory[2][2]
            {
                stor[0].quantity -= resources_for_construction_factory[2][0];
                stor[1].quantity -= resources_for_construction_factory[2][1];
                stor[2].quantity -= resources_for_construction_factory[2][2];

                stor[2].number_of_factories += 1;
            }

            format!("{}", stor[2].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/build/build_factory_rubber")]
    pub fn build_factory_rubber() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[0].quantity >= resources_for_construction_factory[3][0]
                && stor[1].quantity >= resources_for_construction_factory[3][1]
                && stor[2].quantity >= resources_for_construction_factory[3][2]
            {
                stor[0].quantity -= resources_for_construction_factory[3][0];
                stor[1].quantity -= resources_for_construction_factory[3][1];
                stor[2].quantity -= resources_for_construction_factory[3][2];

                stor[3].number_of_factories += 1;
            }

            format!("{}", stor[3].number_of_factories)
        } else {
            String::new()
        }
    }
}

pub mod destroy {
    use crate::*;

    #[put("/game/logic/construction/destroy/destroy_factory_concrete")]
    pub fn destroy_factory_concrete() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[0].number_of_factories > 0 {
                stor[0].number_of_factories -= 1;

                stor[0].quantity += resources_for_construction_factory[0][0] * 0.75;
                stor[1].quantity += resources_for_construction_factory[0][1] * 0.75;
                stor[2].quantity += resources_for_construction_factory[0][2] * 0.75;
                stor[3].quantity += resources_for_construction_factory[0][3] * 0.75;
            }

            format!("{}", stor[0].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/destroy/destroy_factory_wood")]
    pub fn destroy_factory_wood() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[1].number_of_factories > 0 {
                stor[1].number_of_factories -= 1;

                stor[0].quantity += resources_for_construction_factory[1][0] * 0.75;
                stor[1].quantity += resources_for_construction_factory[1][1] * 0.75;
                stor[2].quantity += resources_for_construction_factory[1][2] * 0.75;
                stor[3].quantity += resources_for_construction_factory[1][3] * 0.75;
            }

            format!("{}", stor[1].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/destroy/destroy_factory_iron")]
    pub fn destroy_factory_iron() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[2].number_of_factories > 0 {
                stor[2].number_of_factories -= 1;

                stor[0].quantity += resources_for_construction_factory[2][0] * 0.75;
                stor[1].quantity += resources_for_construction_factory[2][1] * 0.75;
                stor[2].quantity += resources_for_construction_factory[2][2] * 0.75;
                stor[3].quantity += resources_for_construction_factory[2][3] * 0.75;
            }

            format!("{}", stor[2].number_of_factories)
        } else {
            String::new()
        }
    }

    #[put("/game/logic/construction/destroy/destroy_factory_rubber")]
    pub fn destroy_factory_rubber() -> String {
        let mut game = GAME.lock().unwrap();

        let name = game.logic.name_country.clone();

        let resources_for_construction_factory = game.logic.resources_for_construction_factory;

        if let Some((_, country)) = game
            .logic
            .countries
            .iter_mut()
            .find(|(_, country)| country.name == name)
        {
            let stor = &mut country.storage.0;

            if stor[3].number_of_factories > 0 {
                stor[3].number_of_factories -= 1;

                stor[0].quantity += resources_for_construction_factory[3][0] * 0.75;
                stor[1].quantity += resources_for_construction_factory[3][1] * 0.75;
                stor[2].quantity += resources_for_construction_factory[3][2] * 0.75;
                stor[3].quantity += resources_for_construction_factory[3][3] * 0.75;
            }

            format!("{}", stor[3].number_of_factories)
        } else {
            String::new()
        }
    }
}
