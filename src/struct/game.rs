use crate::*;

#[derive(Debug, Clone)]
pub struct GameS {
    pub(crate) logic: LogicS,
    //pub(crate) web: WebS,
}

pub type FactsGame = (FactsLogic); //, FactsWeb);

impl Create for GameS {
    type Output = GameS;
    type Facts = FactsGame;

    fn new(facts: &Self::Facts) -> Self::Output {
        GameS {
            logic: LogicS::new(facts), //.0),
                                       //web: Web::new(facts.1)
        }
    }

    fn default() -> Self::Output {
        GameS {
            logic: LogicS::default(),
            //web: WebS::default(),
        }
    }
}

impl Control for GameS {
    type Facts = ();

    fn update(&mut self, _facts: Self::Facts) {
        self.logic.update(());
    }
}

lazy_static! {
    pub static ref GAME: Mutex<GameS> = Mutex::new(GameS::default());
}

lazy_static! {
    pub static ref GAME_HASH_MAP: Mutex<HashMap<(String, String), GameS>> =
        Mutex::new(HashMap::new());
}
