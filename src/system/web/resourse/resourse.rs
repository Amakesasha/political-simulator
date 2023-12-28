use crate::*;

impl StorageS {
    pub fn resourse(num: usize) -> String {
        let game = GAME.lock().unwrap();

        if let Some(country) =
            CountryS::hashmap_give(&game.logic.countries, &game.logic.name_country[0], false)
        {
            let stor = &country.storage[num];
            let res_for_con = &game.logic.resources_for_construction[num];

            HTML_RESURSE
                .replace("{0_num}", &stor.0[0].quantity.to_string())
                .replace("{0_res}", &stor.html_from_resurse(0, num, res_for_con))
                .replace("{0_build_0}", &res_for_con[0][0].to_string())
                .replace("{0_build_1}", &res_for_con[0][1].to_string())
                .replace("{0_build_2}", &res_for_con[0][2].to_string())
                .replace("{0_prod}", &stor.0[0].production_1_factory.to_string())
                .replace("{1_num}", &stor.0[1].quantity.to_string())
                .replace("{1_res}", &stor.html_from_resurse(1, num, res_for_con))
                .replace("{1_build_0}", &res_for_con[1][0].to_string())
                .replace("{1_build_1}", &res_for_con[1][1].to_string())
                .replace("{1_build_2}", &res_for_con[1][2].to_string())
                .replace("{1_prod}", &stor.0[1].production_1_factory.to_string())
                .replace("{2_num}", &stor.0[2].quantity.to_string())
                .replace("{2_res}", &stor.html_from_resurse(2, num, res_for_con))
                .replace("{2_build_0}", &res_for_con[2][0].to_string())
                .replace("{2_build_1}", &res_for_con[2][1].to_string())
                .replace("{2_build_2}", &res_for_con[2][2].to_string())
                .replace("{2_prod}", &stor.0[2].production_1_factory.to_string())
        } else {
            println!("No country: {:?}", &game.logic.name_country);

            String::new()
        }
    }

    fn html_from_resurse(
        &self,
        resourse_id: usize,
        stor: usize,
        res_for_con: &ResForCons,
    ) -> String {
        let resourse = self.0[resourse_id];

        let (color_0, color_1) = {
            let color_0 = if self.0[0].quantity >= res_for_con[resourse_id][0]
                && self.0[1].quantity >= res_for_con[resourse_id][1]
                && self.0[2].quantity >= res_for_con[resourse_id][2]
            {
                format!("#008000")
            } else {
                format!("#FF0000")
            };

            let color_1 = if resourse.number_of_factory > 0 {
                format!("#008000")
            } else {
                format!("#FF0000")
            };

            (color_0, color_1)
        };

        let name = match resourse_id {
            0 => "Concrete",
            1 => "Wood",
            2 => "Iron",
            _ => " ",
        };

        format!(
            r#"
            <style>
                .build_{stor}_{res} {{
                    padding: 2px 4px;
                    background-color: {color_0};
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 19px;
                }}
                .destroy_{stor}_{res} {{
                    padding: 2px 4px;
                    background-color: {color_1};
                    border: none;
                    border-radius: 0px;
                    cursor: pointer;
                    font-size: 19px;
                }}
            </style>

            <label id="number_of_factory_{stor}_{res}">{number_of_factory}</label>
            <table>
                <button id="build_{stor}_{res}" class="build_{stor}_{res}" type="button" onclick="sendPostRequest_{stor}_{res}_0()">
                    Build Factory {text}
                </button>
            </table>
            <table>
                <button id="destroy_{stor}_{res}" class="destroy_{stor}_{res}" type="button" onclick="sendPostRequest_{stor}_{res}_1()">
                    Destroy Factory {text}
                </button>
            </table>
            

            <script>
                function sendPostRequest_{stor}_{res}_0() {{
                    fetch('/logic/country/control/build/{stor}/{res}', {{
                        method: 'PUT',
                        headers: {{
                            'Content-Type': 'application/x-www-form-urlencoded'
                        }},
                        body: new URLSearchParams({{}}),
                    }})
                    .then(response => response.text())
                    .then(result => {{
                        var labelElement = document.getElementById('number_of_factory_{stor}_{res}');
                        labelElement.textContent = result;
                    }})
                    .catch(error => {{ console.error('Error:', error); }});
                }}

                function sendPostRequest_{stor}_{res}_1() {{
                    fetch('/logic/country/control/destroy/{stor}/{res}', {{
                        method: 'PUT',
                        headers: {{
                            'Content-Type': 'application/x-www-form-urlencoded'
                        }},
                        body: new URLSearchParams({{}}),
                    }})
                    .then(response => response.text())
                    .then(result => {{
                        var labelElement = document.getElementById('number_of_factory_{stor}_{res}');
                        labelElement.textContent = result;
                    }})
                    .catch(error => {{ console.error('Error:', error); }});
                }}
            </script>
        "#,
            stor = stor,
            res = resourse_id,
            color_0 = color_0,
            color_1 = color_1,
            number_of_factory = self.0[resourse_id].number_of_factory,
            text = name,
        )
    }
}

// /logic/country/control/build/{factory_num}{stor_num}
