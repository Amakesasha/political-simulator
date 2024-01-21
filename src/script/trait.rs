use crate::*;

use core::fmt::Debug;

pub trait Create {
    type Output;
    type Facts;

    //

    fn new(facst: &Self::Facts) -> Self::Output;

    fn default() -> Self::Output;

    fn default_facts() -> Self::Facts;

    //

    fn update_create(date: &mut Self::Output, facst: Self::Facts) {
        *date = Self::new(&facst);
    }

    //

    fn vec_new(vector_facts: &Vec<Self::Facts>) -> Vec<Self::Output> {
        let mut vector = Vec::new();

        for facst in vector_facts {
            vector.push(Self::new(facst));
        }

        vector
    }

    fn hash_map_new(
        vector_key: &Vec<String>,
        vector_facts: &Vec<Self::Facts>,
    ) -> HashMap<String, Self::Output> {
        let mut hash_map: HashMap<String, Self::Output> = HashMap::new();

        if vector_key.len() > vector_facts.len() {
            for i in 0..vector_facts.len() {
                hash_map.insert(vector_key[i].clone(), Self::new(&vector_facts[i]));
            }
        } else {
            for i in 0..vector_key.len() {
                hash_map.insert(vector_key[i].clone(), Self::new(&vector_facts[i]));
            }
        }

        hash_map
    }

    //

    fn test()
    where
        <Self as script::r#trait::Create>::Output: PartialEq + Debug,
    {
        let default_0 = Self::default();
        let default_1 = Self::new(&Self::default_facts());

        assert_eq!(default_0, default_1);
    }
}

pub trait Give {
    type Output;
    type ID;

    fn vector_give<'a>(facts: &'a Vec<Self::Output>, id: &'a Self::ID) -> Option<&'a Self::Output>;

    fn vector_give_mut<'a>(
        facts: &'a mut Vec<Self::Output>,
        id: &Self::ID,
    ) -> Option<&'a mut Self::Output>;

    //

    fn hashmap_give<'a>(
        facts: &'a HashMap<Self::ID, Self::Output>,
        id: &'a Self::ID,
        num: bool,
    ) -> Option<&'a Self::Output>;

    fn hashmap_give_mut<'a>(
        facts: &'a mut HashMap<Self::ID, Self::Output>,
        id: &Self::ID,
        num: bool,
    ) -> Option<&'a mut Self::Output>;
}

pub trait Control {
    type Facts;

    fn update(&mut self, facts: &Self::Facts);
}
