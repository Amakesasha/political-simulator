extern crate piston;

pub mod r#struct {
    pub mod gui {
        pub mod gui;
        pub mod button;
        pub mod window;
    }
    pub mod context {
        pub mod button;
        pub mod context;
        pub mod font;
    }
    pub mod logic {
        pub mod date;
        pub mod logic;
        pub mod country {
            pub mod construction;
            pub mod country;
            pub mod storage;
        }
    }
    pub mod game;
    pub mod geometry;
}
pub mod game_date {
    pub mod create;
    pub mod text;
}
pub mod script {
    pub mod rand;
    pub mod r#trait;
}
pub mod system {
    pub mod draw {
        pub mod date;
        pub mod display;
        pub mod draw;
        pub mod text;
    }
    pub mod event;
    pub mod flow;
}

fn main() {
    GameS::flow();
}

pub use piston_window::*;
pub use std::collections::HashMap;

pub use crate::{
    game_date::{create::*, text::*},
    r#struct::{
        gui::{
            gui::*,
            button::*,
            window::*,
        },
        context::{button::*, context::*, font::*},
        logic::{
            country::{
                construction::*,
                country::*,
                storage::{resource::*, *},
            },
            date::*,
            logic::*,
        },
        game::*,
        geometry::{
            position::*, size::*, *
        },
    },
    script::{r#trait::*, rand::*},
    system::{
        draw::{date::*, display::*, draw::*, text::*},
        event::*,
        flow::*,
    },
};
