use crate::*;

impl ElectricityS {
	pub fn html_get() -> String {
		let mut game = GAME.lock().unwrap();

		let main = HTML_ELECTRICITY.to_string();

		let html = {
            if let Some((_, country)) = game
                .logic
                .countries
                .iter()
                .find(|(_, country)| country.name == game.logic.name_country)
            {
                let elec = &country.electricity.0;

                let mut text = HTML_RESURSE;

                let html_with_data = text
                    .replace("{solar_panel_num}", &elec[0].quantity.to_string())
                    .replace("{solar_panel_res}", &country.electricity.html_from_ps(&country, &game, 0))

                    .replace("{coal_ps_num}", &elec[1].quantity.to_string())
                    .replace("{coal_ps_res}", &country.electricity.html_from_ps(&country, &game, 1))



                    .replace("{hydro_ps_num}", &elec[2].quantity.to_string())
                    .replace("{hydro_ps_res}", &country.electricity.html_from_ps(&country, &game, 2))



                    .replace("{nuclear_ps_num}", &elec[3].quantity.to_string())
                    .replace("{nuclear_ps_res}", &country.electricity.html_from_ps(&country, &game, 3));
                html_with_data
            } else {
                println!("No country: {}", &game.logic.name_country);

                format!("")
            }
        };

        html
	}

    fn html_from_ps(&self, country: &CountryS, game: &GameS, resourse_id: usize) -> String {
        let elec = &self.0;
        let stor = &country.storage.0;

        let color_0 = if stor[0].quantity >= game.logic.resources_for_construction_power_stantion[resourse_id][0]
            && stor[1].quantity >= game.logic.resources_for_construction_power_stantion[resourse_id][1]
            && stor[2].quantity >= game.logic.resources_for_construction_power_stantion[resourse_id][2]
        {
            format!("#008000")
        } else {
            format!("#FF0000")
        };

        let color_1 = if elec[resourse_id].quantity > 0 {
            format!("#008000")
        } else {
            format!("#FF0000")
        };

        let date = (
            (color_0, color_1),
            elec[resourse_id].quantity,
            match resourse_id {
                0 => ("solar_panel", "Solar Panel"),
                1 => ("coal_ps", "Coal Power Station"),
                2 => ("hydro_ps", "Hydro Power Station"),
                3 => ("huclear_ps", "Huclear Power Station"),
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