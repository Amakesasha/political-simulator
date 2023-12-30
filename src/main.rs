pub mod game_date {
    pub mod create;
}
pub mod script {
    pub mod error;
    pub mod rand;
    pub mod r#trait;
}
pub mod r#struct {
    pub mod logic {
        pub mod date;
        pub mod logic;
        pub mod country {
            pub mod control;
            pub mod country;
            pub mod storage;
        }
    }
    pub mod game;
    pub mod geometry;
}
pub mod system {
    pub mod flow_control;
    pub mod render;
    pub mod terminal;
}

// 2_713 Lines of script in the project

fn main() {
    Flow::control(&mut stdout());
}

pub use crossterm::{
    cursor::{self, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize,
    },
};

pub use std::{
    cell::Cell,
    collections::HashMap,
    env,
    io::{stdout, Stdout, Write},
    process,
    process::Command,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

pub use rust_decimal::Decimal;

pub use game_date::create::*;
pub use r#struct::{
    game::*,
    geometry::*,
    logic::{
        country::{
            control::*,
            country::*,
            storage::{resource::*, storage::*},
        },
        date::*,
        logic::*,
    },
};
pub use script::{error::*, r#trait::*, rand::*};

pub use system::{flow_control::*, render::*, terminal::*};
