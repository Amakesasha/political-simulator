use crate::*;

#[derive(Debug, Clone)]
pub struct GameS {
    pub(crate) logic: LogicS,
    #[cfg(feature = "gui")]
    pub(crate) gui: GuiS,
}

pub type FactsGame = (FactsLogic, FactsGui);

impl Create for GameS {
    type Output = GameS;
    type Facts = FactsGame;

    fn new(facts: &Self::Facts) -> Self::Output {
        GameS {
            logic: LogicS::new(&facts.0),
            #[cfg(feature = "gui")]
            gui: GuiS::new(&facts.1),
        }
    }

    fn default() -> Self::Output {
        GameS {
            logic: LogicS::default(),
            #[cfg(feature = "gui")]
            gui: GuiS::default(),
        }
    }
}

impl Control for GameS {
    type Facts = ();

    fn update(&mut self, _facts: &Self::Facts) {
        self.logic.update(&());
        self.gui.update(&self.logic);
    }
}

/*
lazy_static! {
    pub static ref GAME: Mutex<GameS> = Mutex::new(GameS::default());
}
*/
