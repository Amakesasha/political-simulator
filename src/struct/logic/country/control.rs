use crate::*;

impl StorageS {
    pub fn build(country: &mut CountryS, logic: &LogicS, stor_num: usize, factory_num: usize, ) {
        let resources = logic.resources_for_construction[stor_num][factory_num];

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

    pub fn destroy(country: &mut CountryS, logic: &LogicS, stor_num: usize, factory_num: usize) {
        let resources = logic.resources_for_construction[stor_num];

        let stor = &mut country.storage;

        if stor[stor_num].0[factory_num].number_of_factory > 0 {
            stor[stor_num].0[factory_num].number_of_factory -= 1;

            stor[0].0[0].quantity += resources[factory_num][0] * 0.75;
            stor[0].0[1].quantity += resources[factory_num][1] * 0.75;
            stor[0].0[2].quantity += resources[factory_num][2] * 0.75;
            stor[0].0[3].quantity += resources[factory_num][3] * 0.75;
        }
    }
}
