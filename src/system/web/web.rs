use crate::*;

pub static HTML_RESURSE: &str = include_str!("resourse/resourse.html");

pub static HTML_LOGIC: &str = include_str!("login/login.html");

pub static HTML_HEADER: &str = include_str!("main/header.html");

pub static HTML_NAVIGATION_BAR: &str = include_str!("main/navigation_bar.html");

#[catch(404)]
pub fn not_found(req: &Request) -> Html<String> {
    let path = req.uri().path();
    Html(format!("Page {} not found.", path))
}

/*
    /login
        /window                           "get"
        /check_password_and_name          "post"

    /logic
        /date
            /next_date                    "post"
        /construction
            /build
                /build_factory_concrete   "post"
                /build_factory_wood       "post"
                /build_factory_iron       "post"
            /destroy
                /destroy_factory_concrete "post"
                /destroy_factory_wood     "post"
                /destroy_factory_iron     "post"
        /table

    /game
        /resourse
        /trade
*/
