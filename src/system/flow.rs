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
        electricity,
        check_password_and_name,
        next_date,


        update_color_button_build,
        update_color_button_destroy,

        update_quantity_resourse,


        build_factory_concrete,
        build_factory_wood,
        build_factory_iron,
        build_factory_rubber,

        destroy_factory_concrete,
        destroy_factory_wood,
        destroy_factory_iron,
        destroy_factory_rubber,
    ]
}
