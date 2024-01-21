use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BufferS {
    pub(crate) buffer: Vec<u32>,
    pub(crate) size: [usize; 2],
}

pub type BufferFacts = [usize; 2];

impl Create for BufferS {
    type Output = BufferS;
    type Facts = BufferFacts;

    fn new(facst: &Self::Facts) -> Self::Output {
        BufferS {
            buffer: vec![0; facst[0] * facst[1]],
            size: *facst,
        }
    }

    fn default() -> Self::Output {
        BufferS {
            buffer: Vec::new(),
            size: [0; 2],
        }
    }

    fn default_facts() -> Self::Facts {
        [0; 2]
    }
}
