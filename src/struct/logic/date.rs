use crate::*;

#[derive(Debug, Clone)]
pub struct DateS {
    pub(crate) draw: bool,
    pub(crate) date: [usize; 3],
    pub(crate) update: [usize; 3],
}

pub type FactsDate = (bool, [usize; 3], [usize; 3]);

impl Create for DateS {
    type Output = DateS;
    type Facts = FactsDate;

    fn new(facts: &Self::Facts) -> Self::Output {
        DateS {
            draw: facts.0,
            date: facts.1,
            update: facts.2,
        }
    }

    fn default() -> Self::Output {
        DateS {
            draw: false,
            date: [0, 0, 0],
            update: [0, 0, 0],
        }
    }
}

impl Control for DateS {
    type Facts = ();

    fn update(&mut self, _facts: Self::Facts) {
        let date = &mut self.date;
        let update = &mut self.update;

        date[0] += update[0];

        if date[0] > 30 {
            date[1] += 1;

            date[0] = 1;
        }

        date[1] += update[1];

        if date[1] > 12 {
            date[2] += 1;

            date[1] = 1;
        }

        date[2] += update[2];
    }
}

impl DateS {
    pub fn date_to_string(&self) -> String {
        let date = &self.date;

        let point = ".".to_string();

        let day = if date[0] < 9 {
            format!("0{}", date[0])
        } else {
            format!("{}", date[0])
        };

        let month = if date[1] < 9 {
            format!("0{}", date[1])
        } else {
            format!("{}", date[1])
        };

        format!("{}{}{}{}{}", day, point, month, point, date[2])
    }
}

#[post("/logic/next_date")]
pub fn next_date() -> Redirect {
    let mut game = GAME.lock().unwrap();

    game.update(());

    Redirect::to("/game")
}
