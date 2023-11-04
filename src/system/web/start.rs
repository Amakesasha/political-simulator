use crate::*;

#[get("/")]
pub fn start() -> Html<String> {
    let mut game = GAME.lock().unwrap();

    let add_factory_tree = r#"
        <style>
            .button-container {
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
            }
            .button {
                position: absolute;
                top: 30px; 
                left: 250px; 
                padding: 10px 20px;
                background-color: #FF0000;
                border: some;
                border-radius: 5px;
                cursor: pointer;
            }
        </style>
        <div class="button-container">
            <form action="/construction/add_factory_tree" method="post">
                <button class="button" type="/construction/add_factory_tree">Add Factory Tree</button>
            </form>
        </div>
    "#;

    let date = format!("<h3>Date: {}</h3>", game.logic.date.date_to_string());

    let resourse = {
        let country = CountryS::give(&game.logic.countries, "Country".to_string()).unwrap();

        let stor = &country.storage;

        format!(
            r#"
            <table>
                <tr>
                    <td> Resource </td> <td> Quantity </td> <td> Number of factories </td>
                </tr>
                <tr>
                    <td> Concrete </td> <td> {} </td> <td> {} </td>
                </tr>
                <tr>
                    <td> Wood </td> <td> {} </td> <td> {} </td>
                </tr>
                <tr>
                    <td> Iron </td> <td> {} </td> <td> {} </td>
                </tr>
            </table>
            "#,
            stor.concrete.quantity,
            stor.concrete.number_of_factories,
            stor.wood.quantity,
            stor.wood.number_of_factories,
            stor.iron.quantity,
            stor.iron.number_of_factories,
        )
    };

    // <link rel="stylesheet" type="text/css" href="/styles.css">

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>Political Simulator</title>
        </head>
        <body>
            <h6>{date}</h6>
            {resourse}
        </body>
        </html>
    "#,
        date = date,
        resourse = resourse
    );

    Html(html)
}
