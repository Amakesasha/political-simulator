#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod r#struct {
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
}
pub mod script {
    pub mod rand;
    pub mod r#trait;
}
pub mod system {
    pub mod web {
        pub mod start;
        pub mod login;
    }
    pub mod flow;
}

fn main() {
    GameS::flow();
}

pub use std::collections::HashMap;
pub use std::sync::Mutex;

pub use rocket::{
    config::{Config, Environment},
    request::{Form, FromParam, FromRequest, Request},
    response::{Redirect, content::Html}
};

pub use lazy_static::lazy_static;

pub use crate::{
    game_date::create::*,
    r#struct::{
        game::*,
        geometry::{aabb::*, position::*, size::*, *},
        logic::{
            country::{
                construction::*,
                country::*,
                storage::{resource::*, *},
            },
            date::*,
            logic::*,
        },
    },
    script::{r#trait::*, rand::*},
    system::{
        flow::*, 
        web::{
            start::*,
            login::*,
        },
    },
};
