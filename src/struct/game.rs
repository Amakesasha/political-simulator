use crate::*;

#[derive(Debug, Clone)]
pub struct GameS {
    pub(crate) logic: LogicS,
    pub(crate) context: ContextS,
}

pub type FactsGame = (FactsLogic, FactsContext);

impl Create for GameS {
    type Output = GameS;
    type Facts = FactsGame;

    fn new(facts: Self::Facts) -> Self::Output {
        GameS {
            logic: LogicS::new(facts.0),
            context: ContextS::new(facts.1),
        }
    }

    fn default() -> Self::Output {
        GameS {
            logic: LogicS::default(),
            context: ContextS::default(),
        }
    }
}

impl Control for GameS {
    type Facts = ();

    fn update(&mut self, _facts: Self::Facts) {
        self.logic.update(());
    }
}
