use crate::*;

#[derive(Debug, Clone)]
pub struct ButtonS {
    pub(crate) w: Button,
    pub(crate) a: Button,
    pub(crate) s: Button,
    pub(crate) d: Button,

    pub(crate) d1: Button,
    pub(crate) d2: Button,
    pub(crate) d3: Button,

    pub(crate) esc: Button,
}

pub type FactsButton = [(Button, bool); 8];

impl Create for ButtonS {
    type Output = ButtonS;
    type Facts = FactsButton;

    fn new(facts: Self::Facts) -> Self::Output {
        ButtonS {
            w: facts[0].0,
            a: facts[1].0,
            s: facts[2].0,
            d: facts[3].0,

            d1: facts[4].0,
            d2: facts[5].0,
            d3: facts[6].0,

            esc: facts[7].0,
        }
    }

    fn default() -> Self::Output {
        ButtonS {
            w: Button::Keyboard(Key::W),
            a: Button::Keyboard(Key::A),
            s: Button::Keyboard(Key::S),
            d: Button::Keyboard(Key::D),

            d1: Button::Keyboard(Key::D1),
            d2: Button::Keyboard(Key::D2),
            d3: Button::Keyboard(Key::D3),
            
            esc: Button::Keyboard(Key::Escape),
        }
    }
}
