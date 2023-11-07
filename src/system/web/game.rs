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
                    <style>
                        .bordered-table {{
                            border: 1px solid black;
                        }}

                        .bordered-table td {{
                            border: 1px solid black;
                        }}
                    </style>

                    <h6>
                        <table class="bordered-table">
                            <tr>
                                <td> Resource </td>
                                <td> Quantity </td>
                                <td> Number of factories </td>
                                <td> Resources Required For Construction </td>
                                <td> Production 1 Plant </td>
                                </tr>
                            <tr>
                                <td> Concrete </td>
                                <td> {} </td>
                                <td> {} </td>
                                <td> 
                                    <table>
                                        <tr>
                                            <td> Concrete: {} </td>
                                            <td> Wood: {} </td>
                                            <td> Iron: {} </td>
                                        </tr>
                                    </table> 
                                </td>
                                <td> {} </td>
                            </tr>
                            <tr>
                                <td> Wood </td>
                                <td> {} </td>
                                <td> {} </td>
                                <td> 
                                    <table>
                                        <tr>
                                            <td> Concrete: {} </td>
                                            <td> Wood: {} </td>
                                            <td> Iron: {} </td>
                                        </tr>
                                    </table> 
                                </td>
                                <td> {} </td>
                            </tr>
                            <tr>
                                <td> Iron </td>
                                <td> {} </td>
                                <td> {} </td>
                                <td> 
                                    <table>
                                        <tr>
                                            <td> Concrete: {} </td>
                                            <td> Wood: {} </td>
                                            <td> Iron: {} </td>
                                        </tr>
                                    </table> 
                                </td>
                                <td> {} </td>
                            </tr>
                        </table>
                    </h6>
                "#,
                stor.concrete.quantity,
                concrete(stor.concrete.number_of_factories),
                stor.concrete.resources_required_for_construction[0],
                stor.concrete.resources_required_for_construction[1],
                stor.concrete.resources_required_for_construction[2],
                stor.concrete.production_1_plant,

                stor.wood.quantity,
                wood(stor.wood.number_of_factories),
                stor.wood.resources_required_for_construction[0],
                stor.wood.resources_required_for_construction[1],
                stor.wood.resources_required_for_construction[2],
                stor.concrete.production_1_plant,

                stor.iron.quantity,
                iron(stor.iron.number_of_factories),
                stor.iron.resources_required_for_construction[0],
                stor.iron.resources_required_for_construction[1],
                stor.iron.resources_required_for_construction[2],
                stor.concrete.production_1_plant,
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
                window.onload = function() {{
                    setTimeout(performPostRequest, 2500);
                }};

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
        </body>
        </html>
    "#,
        date = date,
        resourse = resourse,
    );

    Html(html)
}

fn concrete(date: usize) -> String {
    return format!(r#"
        <style>
            .button {{
                padding: 2px 4px;
                background-color: #00be0a;
                border: none;
                border-radius: 0px;
                cursor: pointer;
                font-size: 8px;
            }}
        </style>

        <label for="concrete-add">{}</label>
        <button id="concrete-add" class="button" type="button" onclick="sendPostRequest_0()">Add Factory Concrete</button>

        <script>
            let isCursorOnButton = false;

            function sendPostRequest_0() {{
                var xhr = new XMLHttpRequest();
                xhr.open("POST", "/construction/add_factory_concrete", true);
                xhr.send(JSON.stringify({{}}));
            }}
        </script>
    "#,
        date
    );
}

fn wood(date: usize) -> String {
    return format!(r#"
        <style>
            .button {{
                padding: 2px 4px;
                background-color: #00be0a;
                border: none;
                border-radius: 0px;
                cursor: pointer;
                font-size: 8px;
            }}

        </style>

        <label for="wood-add">{}</label>
        <button id="wood-add" class="button" type="button" onclick="sendPostRequest_1()">Add Factory Wood</button>

        <script>
            function sendPostRequest_1() {{
                var xhr = new XMLHttpRequest();
                xhr.open("POST", "/construction/add_factory_wood", true);
                xhr.send(JSON.stringify({{}}));
            }}
        </script>
    "#,
        date
    );
}

fn iron(date: usize) -> String {
    return format!(r#"
        <style>
            .button {{
                padding: 2px 4px;
                background-color: #00be0a;
                border: none;
                border-radius: 0px;
                cursor: pointer;
                font-size: 8px;
            }}
        </style>

        <label for="iron-add">{}</label>
        <button id="iron-add" class="button" type="button" onclick="sendPostRequest_2()">Add Factory Iron</button>

        <script>
            function sendPostRequest_2() {{
                var xhr = new XMLHttpRequest();
                xhr.open("POST", "/construction/add_factory_iron", true);
                xhr.send(JSON.stringify({{}}));
            }}
        </script>
    "#,
        date
    );
}