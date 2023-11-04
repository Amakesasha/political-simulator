use crate::*;

#[get("/")]
pub fn login() -> Html<String> {
    let mut game = GAME.lock().unwrap();

    let login = r#"
        <script>
            function submitForm(event) {
                event.preventDefault();
                
                const username = document.getElementById('input1').value;
                const password = document.getElementById('input2').value;

                fetch('/login/check_password_and_name', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded'
                    },
                    body: new URLSearchParams({
                        'username': username,
                        'password': password
                    })
                }).then(response => {
                    // Handle the response from the server
                }).catch(error => {
                    // Handle errors
                });
            }
            
            document.getElementById('button').addEventListener('click', submitForm);
        </script>
        
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
            <form action="/login/check_password_and_name" method="post">
                <input type="text" id="input1" class="input" name="username" placeholder="UserName">
                <input type="text" id="input2" class="input" name="password" placeholder="Password">
                <button id="button" class="button">Input</button>
            </form>
        </div>
    "#;

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>Political Simulator</title>
        </head>
        <body>
            {login}
        </body>
        </html>
    "#,
        login = login
    );

    Html(html)
}

#[post("/login/check_password_and_name", data = "<facst>")]
pub fn check_password_and_name(facst: Form<LoginForm>) -> Redirect {
	print!("Check Password and Name: {:?} ", facst.0);

	let mut hash_map = GAME_HASH_MAP.lock().unwrap(); 

	if let Some(game) = hash_map.get(&(facst.0.username.clone(), facst.0.password.clone())) {
		*GAME.lock().unwrap() = game.clone();

		println!("True");

		Redirect::to("/game")
	} else {
		println!("False");

		Redirect::to("/")
	}
}

#[derive(Debug, Clone, FromForm)]
struct LoginForm {
    password: String,
	username: String,
}
