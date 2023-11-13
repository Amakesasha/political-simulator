use crate::*;

#[get("/game/draw/resourse")]
pub fn resourse() -> Html<String> {
    let navigation_bar = HTML_NAVIGATION_BAR.to_string();

    let main = StorageS::html_get().to_string();

    let mut html = String::new();

    {
        let game = GAME.lock().unwrap();
        html = html_get_main(game.logic.date.date_to_string(), navigation_bar, main);
    }

    Html(html)
}

#[get("/game/draw/electricity")]
pub fn electricity() -> Html<String> {
    let navigation_bar = HTML_NAVIGATION_BAR.to_string();

    let main = ElectricityS::html_get().to_string();

    let mut html = String::new();
    
    {
        let game = GAME.lock().unwrap();
        html = html_get_main(game.logic.date.date_to_string(), navigation_bar, main);
    }

    Html(html)
}

pub fn html_get_main(date: String, navigation_bar: String, main: String) -> String {
    return format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>Political Simulator</title>

                <script>
                    window.onload = function() {{
                        setTimeout(performPostRequest, 1000);
                    }};

                    function performPostRequest() {{
                        const path = location.pathname;

                        fetch('/game/logic/date/next_date', {{
                            method: 'POST',
                            headers: {{ 'Content-Type': 'application/x-www-form-urlencoded' }},
                            body: new URLSearchParams({{ a: path }})
                        }})
                        .then(response => {{
                            function changeText() {{
                                var element = document.getElementById("myElement");
                                element.innerHTML = "<strong>Новый текст</strong>";
                            }}

                            location.reload(path);
                        }})
                        .catch(error => {{
                            location.reload(path);
                        }});
                    }}
                </script>

                <style>
                    div {{
                        margin: 10px;
                        border: 1px solid black;
                        font-size: 20px;
                        height: 80px;
                    }}
                    #header {{
                        background-color: #ccc;
                    }}
                    #leftSidebar {{
                        background-color: #f4f4f4;
                        float: left;
                        width: 150px;
                        height: 350px;
                    }}
                    
                    #main {{
                        background-color: #FFFFFF;
                        height: 400px;
                        margin-left: 190px;
                        margin-right: 10px;
                    }}
                    #footer {{
                        background-color: #ccc;
                    }}
                </style>
            </head>
            <body>
                <div id="header">
                    {date}
                </div>
                <div id="leftSidebar"> 
                    {navigation_bar} 
                </div>
                <div id="main"> 
                    {main} 
                </div>
                <div id="footer"> 

                </div>
            </body>
        </html>
    "#,
        date = date,
        navigation_bar = navigation_bar,
        main = main
    );
}

/*
                    #rightSidebar {{
                        background-color: #f4f4f4;
                        float: right;
                        width: 150px;
                        height: 350px;
                    }}

                    <div id="rightSidebar">  

                    </div>
*/