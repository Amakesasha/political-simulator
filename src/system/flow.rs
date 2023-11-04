use crate::*;

impl GameS {
    pub fn flow() {
        {
            let mut game = GAME_HASH_MAP.lock().unwrap();

            *game = GameS::give_date_hash_map();
        }

        println!("Please open this link: http://localhost:8000/");

        let config = Config::build(Environment::Development)
            .log_level(rocket::config::LoggingLevel::Off) // Отключить вывод в терминал
            .finalize()
            .unwrap();

        rocket::custom(config).mount("/", routes()).launch();
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        game,
        check_password_and_name,
        add_factory_tree,
        add_factory_iron,
        add_factory_concrete
    ]
}
