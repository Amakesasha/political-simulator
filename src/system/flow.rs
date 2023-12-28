use crate::*;

impl GameS {
    pub fn flow() {
        {
            let mut game = GAME.lock().unwrap();

            GameS::update_create(&mut game, GameS::give_date());
        }

        println!("Please open this link: http://localhost:8000/login/window");

        let config = Config::build(Environment::Development)
            .log_level(rocket::config::LoggingLevel::Off)
            .finalize()
            .unwrap();

        rocket::custom(config)
            .mount("/", routes())
            .register(catchers![not_found])
            .launch();
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login, 
        resourse,
        next_date,
        check_password_and_name,
        build,
        destroy
    ]
}
