use crate::*;

#[get("/game/resourse")]
pub fn resourse() -> Html<String> {
    let navigation_bar = HTML_NAVIGATION_BAR.to_string();

    let main = StorageS::resourse(0).to_string();

    let footer = HTML_HEADER.to_string();

    let html = html_get_main(navigation_bar, main, footer);

    Html(html)
}

pub fn html_get_main(navigation_bar: String, main: String, header: String) -> String {
    return format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>Political Simulator</title>

                <script>
                    window.onload = function() {{
                        setInterval(performPostRequest, 1000);
                    }};

                    function performPostRequest() {{
                        fetch('/logic/date/next_date', {{
                            method: 'PUT',
                            headers: {{
                                'Content-Type': 'application/x-www-form-urlencoded'
                            }},
                            body: new URLSearchParams({{}}),
                        }})
                        .then(response => response.text())
                        .then(result => {{
                            var labelElement = document.getElementById('date_text');
                            labelElement.textContent = result;
                        }})
                        .catch(error => {{ console.error('Error:', error); }});
                    }}

                    function Start_1_Second() {{
                        performPostRequest()
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
                    {header}
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
        navigation_bar = navigation_bar,
        main = main,
        header = header,
    );
}
