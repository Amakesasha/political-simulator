use crate::*;

#[get("/game")]
pub fn game() -> Html<String> {
    let mut game = GAME.lock().unwrap();

    let date = if game.logic.date.draw {
        format!("<h7>Date: {}</h7>", game.logic.date.date_to_string())
    } else {
        format!("")
    };

    let resourse = {
        if let Some((_, country)) = game
            .logic
            .countries
            .iter()
            .find(|(_, country)| country.name == game.logic.name_country)
        {
            let stor = &country.storage;

            format!(
                r#"
                    <h6>
                        <table>
                            <tr>
                                <td> Resource </td> 
                                <td> Quantity </td> 
                                <td> Number of factories </td>
                                <td> Production 1 Plant </td>
                            </tr>
                            <tr>
                                <td> Concrete </td> <td> {} </td> <td> {} </td> <td> {} </td>
                            </tr>
                            <tr>
                                <td> Wood </td> <td> {} </td> <td> {} </td> <td> {} </td>
                            </tr>
                            <tr>
                                <td> Iron </td> <td> {} </td> <td> {} </td> <td> {} </td>
                            </tr>
                        </table>
                    </h6>
                "#,
                stor.concrete.quantity,
                stor.concrete.number_of_factories,
                stor.concrete.production_1_plant,
                stor.wood.quantity,
                stor.wood.number_of_factories,
                stor.wood.production_1_plant,
                stor.iron.quantity,
                stor.iron.number_of_factories,
                stor.iron.production_1_plant,
            )
        } else {
            println!("No country: {}", &game.logic.name_country);

            format!("")
        }
    };

    // <link rel="stylesheet" type="text/css" href="/styles.css">

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>Political Simulator</title>
            <script>
                // Запускаем выполнение POST-запроса через 5 секунд после загрузки страницы
                window.onload = function() {{
                    setTimeout(performPostRequest, 5000);
                }};

                // Функция для выполнения POST-запроса
                function performPostRequest() {{
                    var formElement = document.createElement('form');
                    formElement.action = '/logic/next_date';
                    formElement.method = 'post';
                    document.body.appendChild(formElement);
                    formElement.submit();
                }}
            </script>
        </head>
        <body>
            {date}
            {resourse}
            {wood} {iron} {concrete}
        </body>
        </html>
    "#,
        date = date,
        resourse = resourse,
        wood = wood(), iron = iron(), concrete = concrete()
    );

    Html(html)
}

fn wood() -> String {
    return format!(
        r#"
            <style>
                .wood-button-container {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }}
                .wood-button {{
                    position: absolute;
                    top: 71px; 
                    left: 111px; 
                    padding: 2px 4px;
                    background-color: #00be0a;
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 8px;
                }}
            </style>
            <div class="wood-button-container">
                <form action="/construction/add_factory_wood" method="post">
                    <button class="wood-button" type="submit">Add Factory Wood</button>
                </form>
            </div>
        "#
    );
}

fn iron() -> String {
    return format!(
        r#"
            <style>
                .iron-button-container {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }}
                .iron-button {{
                    position: absolute;
                    top: 87px; 
                    left: 111px; 
                    padding: 2px 4px;
                    background-color: #00be0a;
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 8px;
                }}
            </style>
            <div class="iron-button-container">
                <form action="/construction/add_factory_iron" method="post">
                    <button class="iron-button" type="/construction/add_factory_iron">Add Factory Iron</button>
                </form>
            </div>
        "#
    );
}

fn concrete() -> String {
    return format!(
        r#"
            <style>
                .concrete-button-container {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }}
                .concrete-button {{
                    position: absolute;
                    top: 103px; 
                    left: 111px; 
                    padding: 2px 4px;
                    background-color: #00be0a;
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 8px;
                }}
            </style>
            <div class="concrete-button-container">
                <form action="/construction/add_factory_concrete" method="post">
                    <button class="concrete-button" type="/construction/add_factory_concrete">Add Factory Concrete</button>
                </form>
            </div>
        "#
    );
}