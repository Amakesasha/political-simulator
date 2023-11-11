use crate::*;

impl StorageS {
    pub fn resourse() -> String {
        let mut game = GAME.lock().unwrap();

        let html = {
            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let stor = &country.storage;

                let mut text = HTML_RESURSE;

                let html_with_data = text
                    .replace("{conc_num}", &stor.concrete.quantity.to_string())
                    .replace("{conc_res}", &stor.html_from_resurse(0).to_string())
                    .replace(
                        "{conc_build_0}",
                        &stor.concrete.resources_required_for_construction[0].to_string(),
                    )
                    .replace(
                        "{conc_build_1}",
                        &stor.concrete.resources_required_for_construction[1].to_string(),
                    )
                    .replace(
                        "{conc_build_2}",
                        &stor.concrete.resources_required_for_construction[2].to_string(),
                    )
                    .replace("{conc_prod}", &stor.concrete.production_1_plant.to_string())
                    .replace("{wood_num}", &stor.wood.quantity.to_string())
                    .replace("{wood_res}", &stor.html_from_resurse(1).to_string())
                    .replace(
                        "{wood_build_0}",
                        &stor.wood.resources_required_for_construction[0].to_string(),
                    )
                    .replace(
                        "{wood_build_1}",
                        &stor.wood.resources_required_for_construction[1].to_string(),
                    )
                    .replace(
                        "{wood_build_2}",
                        &stor.wood.resources_required_for_construction[2].to_string(),
                    )
                    .replace("{wood_prod}", &stor.concrete.production_1_plant.to_string())
                    .replace("{iron_num}", &stor.iron.quantity.to_string())
                    .replace("{iron_res}", &stor.html_from_resurse(2).to_string())
                    .replace(
                        "{iron_build_0}",
                        &stor.iron.resources_required_for_construction[0].to_string(),
                    )
                    .replace(
                        "{iron_build_1}",
                        &stor.iron.resources_required_for_construction[1].to_string(),
                    )
                    .replace(
                        "{iron_build_2}",
                        &stor.iron.resources_required_for_construction[2].to_string(),
                    )
                    .replace("{iron_prod}", &stor.concrete.production_1_plant.to_string());

                html_with_data
            } else {
                println!("No country: {}", &game.logic.name_country);

                format!("")
            }
        };

        html
    }

    fn html_from_resurse(&self, resourse_id: u8) -> String {
        let resourse = match resourse_id {
            0 => &self.concrete,
            1 => &self.wood,
            2 => &self.iron,
            _ => &self.concrete,
        };

        let (color_0, color_1) = {
            let color_0 = if self.concrete.quantity
                >= resourse.resources_required_for_construction[0]
                && self.wood.quantity >= resourse.resources_required_for_construction[1]
                && self.iron.quantity >= resourse.resources_required_for_construction[2]
            {
                format!("#008000")
            } else {
                format!("#FF0000")
            };

            let color_1 = if resourse.number_of_factories > 0 {
                format!("#008000")
            } else {
                format!("#FF0000")
            };

            (color_0, color_1)
        };

        let date = match resourse_id {
            0 => (
                color_0,
                color_1,
                resourse.number_of_factories,
                "concrete",
                "Concrete",
            ),
            1 => (
                color_0,
                color_1,
                resourse.number_of_factories,
                "wood",
                "Wood",
            ),
            2 => (
                color_0,
                color_1,
                resourse.number_of_factories,
                "iron",
                "Iron",
            ),
            _ => (color_0, color_1, resourse.number_of_factories, " ", " "),
        };

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
                    var xhr = new XMLHttpRequest();
                    xhr.open("POST", "/logic/construction/build/build_factory_{post}", true);
                    xhr.send(JSON.stringify({{}}));
                }}

                function sendPostRequest_{resourse_id}_1() {{
                    var xhr = new XMLHttpRequest();
                    xhr.open("POST", "/logic/construction/destroy/destroy_factory_{post}",true);
                    xhr.send(JSON.stringify({{}}));
                }}
            </script>
        "#,
            resourse_id = resourse_id,
            color_0 = date.0,
            color_1 = date.1,
            number_of_factories = date.2,
            post = date.3,
            text = date.4,
        )
    }
}