pub trait Create {
    type Output;
    type Facts;

    fn new(facts: Self::Facts) -> Self::Output;

    fn vec_new(vector_facts: Vec<Self::Facts>) -> Vec<Self::Output> {
        let mut vector = Vec::new();

        for facts in vector_facts {
            vector.push(Self::new(facts));
        }

        vector
    }

    fn default() -> Self::Output;
}

pub trait Give {
    type Output;
    type ID;

    fn give(facts: &Vec<Self::Output>, id: Self::ID) -> Option<Self::Output>;
    fn give_mut<'a>(facts: &'a mut Vec<Self::Output>, id: Self::ID)
        -> Option<&'a mut Self::Output>;
}

pub trait Control {
    type Facts;

    fn update(&mut self, facts: Self::Facts);
}
