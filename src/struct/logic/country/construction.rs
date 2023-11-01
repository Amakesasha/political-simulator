use crate::*;

impl CountryS {
    pub fn add_factory_tree(&mut self) {
        let stor = &mut self.storage;

        if stor.concrete.quantity >= stor.tree.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.tree.resources_required_for_construction[1]
            && stor.tree.quantity >= stor.tree.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.tree.resources_required_for_construction[2];
            stor.iron.quantity -= stor.tree.resources_required_for_construction[1];
            stor.tree.quantity -= stor.tree.resources_required_for_construction[0];

            stor.tree.number_of_factories += 1;
        }
    }

    pub fn add_factory_iron(&mut self) {
        let stor = &mut self.storage;

        if stor.concrete.quantity >= stor.iron.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.iron.resources_required_for_construction[1]
            && stor.tree.quantity >= stor.iron.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.iron.resources_required_for_construction[2];
            stor.iron.quantity -= stor.iron.resources_required_for_construction[1];
            stor.tree.quantity -= stor.iron.resources_required_for_construction[0];

            stor.iron.number_of_factories += 1;
        }
    }

    pub fn add_factory_concrete(&mut self) {
        let stor = &mut self.storage;

        if stor.concrete.quantity >= stor.concrete.resources_required_for_construction[2]
            && stor.iron.quantity >= stor.concrete.resources_required_for_construction[1]
            && stor.tree.quantity >= stor.concrete.resources_required_for_construction[0]
        {
            stor.concrete.quantity -= stor.concrete.resources_required_for_construction[2];
            stor.iron.quantity -= stor.concrete.resources_required_for_construction[1];
            stor.tree.quantity -= stor.concrete.resources_required_for_construction[0];

            stor.concrete.number_of_factories += 1;
        }
    }
}
