use crate::*;

impl GameS {
    pub fn flow() {
        {
            let mut game = GAME.lock().unwrap();

            GameS::update_create(&mut game, GameS::give_date());
        }

        println!("Please open this link: http://localhost:8000/login/window");

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
        next_date,
        add_factory_wood,
        add_factory_iron,
        add_factory_concrete
    ]
}
