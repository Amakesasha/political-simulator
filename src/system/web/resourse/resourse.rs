use crate::*;

impl StorageS {
    pub fn html_get() -> String {
        let mut game = GAME.lock().unwrap();

        let html = {
            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let stor = &country.storage.0;

                let mut text = HTML_RESURSE;

                let html_with_data = text
                    .replace("{conc_num}", &stor[0].quantity.to_string())
                    .replace("{conc_res}", &country.storage.html_from_resurse(&game, 0))
                    .replace(
                        "{conc_table}",
                        &Self::resourse_table(game.logic.resources_for_construction_factory, 3)
                    )
                    .replace("{conc_prod}", &stor[0].production_1_plant.to_string())



                    .replace("{wood_num}", &stor[1].quantity.to_string())
                    .replace("{wood_res}", &country.storage.html_from_resurse(&game, 1))
                    .replace(
                        "{wood_table}",
                        &Self::resourse_table(game.logic.resources_for_construction_factory, 3)
                    )
                    .replace("{wood_prod}", &stor[1].production_1_plant.to_string())



                    .replace("{iron_num}", &stor[2].quantity.to_string())
                    .replace("{iron_res}", &country.storage.html_from_resurse(&game, 2))
                    .replace(
                        "{iron_table}",
                        &Self::resourse_table(game.logic.resources_for_construction_factory, 3)
                    )
                    .replace("{iron_prod}", &stor[2].production_1_plant.to_string())



                    .replace("{rubb_num}", &stor[3].quantity.to_string())
                    .replace("{rubb_res}", &country.storage.html_from_resurse(&game, 3))
                    .replace(
                        "{rubb_table}",
                        &Self::resourse_table(game.logic.resources_for_construction_factory, 3)
                    )
                    .replace("{rubb_prod}", &stor[3].production_1_plant.to_string());

                html_with_data
            } else {
                println!("No country: {}", &game.logic.name_country);

                format!("")
            }
        };

        html
    }

    fn resourse_table(date: [[f64; 4]; 4], resourse_id: usize) -> String {
        let td_0 = if date[resourse_id][0] > 0.0 {
            format!("<td> Concrete: {} </td>", date[resourse_id][0])
        } else {
            format!("")
        };

        let td_1 = if date[resourse_id][1] > 0.0 {
            format!("<td> Wood: {} </td>", date[resourse_id][1])
        } else {
            format!("")
        };

        let td_2 = if date[resourse_id][2] > 0.0 {
            format!("<td> Iron: {} </td>", date[resourse_id][2])
        } else {
            format!("")
        };

        let td_3 = if date[resourse_id][3] > 0.0 {
            format!("<td> Rubber: {} </td>", date[resourse_id][3])
        } else {
            format!("")
        };

        format!(
            "<tr>
                {}
                {}
                {}
                {}
            </tr>",
            td_0, td_1, td_2, td_3
        )
    }

    fn html_from_resurse(&self, game: &GameS, resourse_id: usize) -> String {
        let stor = &self.0;

        let color_0 = if stor[0].quantity >= game.logic.resources_for_construction_factory[resourse_id][0]
            && stor[1].quantity >= game.logic.resources_for_construction_factory[resourse_id][1]
            && stor[2].quantity >= game.logic.resources_for_construction_factory[resourse_id][2]
        {
            format!("#008000")
        } else {
            format!("#FF0000")
        };

        let color_1 = if stor[resourse_id].number_of_factories > 0 {
            format!("#008000")
        } else {
            format!("#FF0000")
        };

        let date = (
            (color_0, color_1),
            stor[resourse_id].number_of_factories,
            match resourse_id {
                0 => ("concrete", "Concrete"),
                1 => ("wood", "Wood"),
                2 => ("iron", "Iron"),
                3 => ("rubber", "Rubber"),
                _ => ("_", "_"),
            }
            ,
        );

        format!(
            r#"
            <style>
                .{post}-build {{
                    padding: 2px 4px;
                    background-color: {color_0};
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 19px;
                }}
                .{post}-destroy {{
                    padding: 2px 4px;
                    background-color: {color_1};
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 19px;
                }}
            </style>

            <label for="{post}-build">{number_of_factories}</label>
            <table>
                <button id="{post}-build" class="{post}-build" type="button" onclick="sendPostRequest_{resourse_id}_0()">
                    Build Factory {text}
                </button>
            </table>
            <table>
                <button id="{post}-destroy" class="{post}-destroy" type="button" onclick="sendPostRequest_{resourse_id}_1()">
                    Destroy Factory {text}
                </button>
            </table>
            

            <script>
                function sendPostRequest_{resourse_id}_0() {{
                    const path = location.pathname;

                    fetch('/game/logic/construction/build/build_factory_{post}', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/x-www-form-urlencoded' }},
                        body: new URLSearchParams({{ a: path }})
                    }})
                    .then(response => {{}})
                    .catch(error => {{}});
                }}

                function sendPostRequest_{resourse_id}_1() {{
                    const path = location.pathname;

                    fetch('/game/logic/construction/destroy/destroy_factory_{post}', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/x-www-form-urlencoded' }},
                        body: new URLSearchParams({{ a: path }})
                    }})
                }}
            </script>
        "#,
            resourse_id = resourse_id,
            color_0 = date.0.0,
            color_1 = date.0.1,
            number_of_factories = date.1,
            post = date.2.0,
            text = date.2.1,
        )
    }
}
