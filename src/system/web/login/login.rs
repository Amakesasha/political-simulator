use crate::*;

#[get("/game/login/window")]
pub fn login() -> Html<String> {
    let html = HTML_LOGIC;

    Html(html.to_string())
}

#[post("/game/login/check_password_and_name", data = "<facst_0_0>")]
pub fn check_password_and_name(facst_0_0: Form<LoginForm>) -> Redirect {
    print!("Input in Country: {:?} ", facst_0_0.0);

    let mut game = GAME.lock().unwrap();

    if let Some(country) = game.logic.countries.get_mut(&facst_0_0.0.password.clone()) {
        if country.name == facst_0_0.0.username {
            game.logic.name_country = facst_0_0.0.username;

            println!("True");

            Redirect::to("/game/draw/resourse")
        } else {
            println!("False");

            Redirect::to("/game/login/window")
        }
    } else {
        println!("False");

        Redirect::to("/game/login/window")
    }
}

#[derive(Debug, Clone, FromForm)]
struct LoginForm {
    password: String,
    username: String,
}
