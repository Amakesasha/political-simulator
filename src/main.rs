#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

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
        pub mod web;
        pub mod resourse {
            pub mod resourse;
        }
        pub mod login {
            pub mod login;
        }
        pub mod r#main {
            pub mod r#main;
        }
    }
    pub mod flow;
}

fn main() {
    GameS::flow();
}

pub use std::{collections::HashMap, fs::File, io::prelude::*, sync::Mutex};

pub use rocket::{
    config::{Config, Environment},
    request::{Form, FromParam, FromRequest, Request},
    response::{content::Html, Redirect},
};

pub use lazy_static::lazy_static;

pub use crate::{
    game_date::create::*,
    r#struct::{
        game::*,
        logic::{
            country::{
                control::*,
                country::*,
                storage::{resource::*, storage::*},
            },
            date::*,
            logic::*,
        },
    },
    script::{r#trait::*, rand::*},
    system::{
        flow::*,
        web::{login::login::*, r#main::r#main::*, resourse::resourse::*, web::*},
    },
};
