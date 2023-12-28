use crate::*;

#[put("/logic/country/control/build/<stor_num>/<factory_num>")]
pub fn build(stor_num: usize, factory_num: usize) -> String {
    let mut game = GAME.lock().unwrap();

    let logic = game.logic.clone();

    if let Some(ref mut country) =
        CountryS::hashmap_give_mut(&mut game.logic.countries, &logic.name_country[0], false)
    {        
        StorageS::build(country, &logic.resources_for_construction, stor_num, factory_num);

        format!("{}", country.storage[stor_num].0[factory_num].number_of_factory)
    } else {
        String::new()
    }
}

#[put("/logic/country/control/destroy/<stor_num>/<factory_num>")]
pub fn destroy(stor_num: usize, factory_num: usize) -> String {
    let mut game = GAME.lock().unwrap();

    let logic = game.logic.clone();

    if let Some(ref mut country) =
        CountryS::hashmap_give_mut(&mut game.logic.countries, &logic.name_country[0], false)
    {
        StorageS::destroy(country, &logic.resources_for_construction, stor_num, factory_num);

        format!("{}", country.storage[stor_num].0[factory_num].number_of_factory)
    } else {
        String::new()
    }
}

impl StorageS {
    pub fn build(
        country: &mut CountryS, 
        resources: &[ResForCons; NUM_STOR + 1], 
        factory_num: usize, 
        stor_num: usize
    ) {
        let resources = resources[stor_num][factory_num];

        let stor = &mut country.storage;

        if stor[0].0[0].quantity >= resources[0]
            && stor[0].0[1].quantity >= resources[1]
            && stor[0].0[2].quantity >= resources[2]
            && stor[0].0[3].quantity >= resources[3]
        {
            stor[0].0[0].quantity -= resources[0];
            stor[0].0[1].quantity -= resources[1];
            stor[0].0[2].quantity -= resources[2];
            stor[0].0[3].quantity -= resources[3];

            stor[stor_num].0[factory_num].number_of_factory += 1;
        }
    }

    pub fn destroy(
        country: &mut CountryS, 
        resources: &[ResForCons; NUM_STOR + 1], 
        factory_num: usize, 
        stor_num: usize
    ) {
        let resources = resources[stor_num][factory_num];

        let stor = &mut country.storage;

        if stor[stor_num].0[factory_num].number_of_factory > 0 {
            stor[stor_num].0[factory_num].number_of_factory -= 1;

            stor[0].0[0].quantity += resources[0] * 0.75;
            stor[0].0[1].quantity += resources[1] * 0.75;
            stor[0].0[2].quantity += resources[2] * 0.75;
            stor[0].0[3].quantity += resources[3] * 0.75;
        }
    }
}
