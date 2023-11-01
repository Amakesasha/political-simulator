use crate::*;

#[derive(Debug, Clone)]
pub struct ContextS {
    pub(crate) play_stop: [bool; 2],
    pub(crate) button: ButtonS,
    pub(crate) font: FontS,
}

pub type FactsContext = (bool, bool, FactsButton);

impl Create for ContextS {
    type Output = ContextS;
    type Facts = FactsContext;

    fn new(facts: Self::Facts) -> Self::Output {
        ContextS {
            play_stop: [facts.0, facts.1],
            button: ButtonS::new(facts.2),
            font: FontS::new(()),
        }
    }

    fn default() -> Self::Output {
        ContextS {
            play_stop: [true, true],
            button: ButtonS::default(),
            font: FontS::default(),
        }
    }
}
