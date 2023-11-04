use crate::*;

impl GameS {
    pub fn flow() {
        {
            let mut date = GAME.lock().unwrap();

            GameS::update_create(&mut date, GameS::give_date());
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
        start,
        add_factory_tree,
        /*add_factory_iron,
        add_factory_concrete*/
    ]
}
