use crate::*;

#[get("/game/resourse")]
pub fn resourse() -> Html<String> {
    let navigation_bar = HTML_NAVIGATION_BAR.to_string();

    let main = StorageS::resourse().to_string();

    let html = html_get_main(navigation_bar, main);

    Html(html)
}

pub fn html_get_main(navigation_bar: String, main: String) -> String {
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
                        var formElement = document.createElement('form');
                        formElement.action = '/logic/date/next_date';
                        formElement.method = 'post';
                        document.body.appendChild(formElement);
                        formElement.submit();
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
                    #rightSidebar {{
                        background-color: #f4f4f4;
                        float: right;
                        width: 150px;
                        height: 350px;
                    }}
                    #main {{
                        background-color: #FFFFFF;
                        height: 400px;
                        margin-left: 190px;
                        margin-right: 190px;
                    }}
                    #footer {{
                        background-color: #ccc;
                    }}
                </style>
            </head>
            <body>
                <div id="header"> 

                </div>
                <div id="leftSidebar"> 
                    {navigation_bar} 
                </div>
                <div id="rightSidebar">  

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
        main = main
    );
}
