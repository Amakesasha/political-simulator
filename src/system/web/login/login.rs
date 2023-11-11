use crate::*;

#[get("/login/window")]
pub fn login() -> Html<String> {
    let html = HTML_LOGIC;

    Html(html.to_string())
}

#[post("/login/check_password_and_name", data = "<facst>")]
pub fn check_password_and_name(facst: Form<LoginForm>) -> Redirect {
    print!("Input in Country: {:?} ", facst.0);

    let mut game = GAME.lock().unwrap();

    if let Some(country) = game.logic.countries.get_mut(&facst.0.password.clone()) {
        if country.name == facst.0.username {
            game.logic.name_country = facst.0.username;

            println!("True");

            Redirect::to("/game/resourse")
        } else {
            println!("False");

            Redirect::to("/login/window")
        }
    } else {
        println!("False");

        Redirect::to("/login/window")
    }
}

#[derive(Debug, Clone, FromForm)]
struct LoginForm {
    password: String,
    username: String,
}
