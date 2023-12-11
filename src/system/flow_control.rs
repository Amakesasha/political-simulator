use crate::*;

pub struct Flow;

impl Flow {
    pub fn control(stdout: &mut Stdout) {
        Terminal::on_off(true, true, stdout);

        let mut game = GameS::new(&GameS::give_date());

        let mut last_output_time = Instant::now();

        loop {
            let now = Instant::now();
            let elapsed = now.duration_since(last_output_time);

            if elapsed >= Duration::from_millis(1000) {
                Flow::render(&game, stdout);

                last_output_time = now;
            }

            if event::poll(Duration::from_millis(250)).error() {
                Flow::event_check(&mut game, stdout);
            }
        }
    }

    pub fn render(game: &GameS, stdout: &mut Stdout) {
        unsafe {
            if !GR {
                Terminal::clean(stdout);

                GuiS::render(&game.gui, stdout);

                GR = true;
            }
        }
    }

    pub fn event_check(game: &mut GameS, stdout: &mut Stdout) {
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            match code {
                KeyCode::Esc => {
                    Terminal::on_off(false, false, stdout);

                    process::exit(0);
                }
                KeyCode::Tab => unsafe {
                    GR = false;
                },
                _ => {}
            }

            let gui = game.gui.clone();

            if let Some(path) = PathS::vector_give(&game.gui.path, code) {
                match &path.name {
                    Ok(name) => {
                        if let Some(button) = ButtonS::vector_give(&gui.button, name.clone()) {
                            ActionE::check_action(game, &button.action);
                        }
                    }
                    Err(name) => {
                        if let Some(window) = WindowS::vector_give(&gui.window, name[0].clone()) {
                            if let Some(button) =
                                ButtonS::vector_give(&window.button, name[1].clone())
                            {
                                ActionE::check_action(game, &button.action);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ActionE {
    pub fn check_action(game: &mut GameS, action_vector: &Vec<ActionE>) {
        for action in action_vector {
            match action {
                ActionE::OpenButton(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        ButtonS::open_all_close(&mut game.gui.button, &meaning);
                    }
                    GuiActionE::OpenRestClose(name, meaning) => {
                        ButtonS::open_rest_close(&mut game.gui.button, &name, &meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        ButtonS::open_one_close(&mut game.gui.button, &name, &meaning);
                    }
                },
                ActionE::OpenWindow(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        WindowS::open_all_close(&mut game.gui.window, &meaning);
                    }
                    GuiActionE::OpenRestClose(name, meaning) => {
                        WindowS::open_rest_close(&mut game.gui.window, &name, &meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        WindowS::open_one_close(&mut game.gui.window, &name, &meaning);
                    }
                },
                ActionE::OpenTable(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        TableS::open_all_close(&mut game.gui.table, &meaning);
                    }
                    GuiActionE::OpenRestClose(name, meaning) => {
                        TableS::open_rest_close(&mut game.gui.table, &name, &meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        TableS::open_one_close(&mut game.gui.table, &name, &meaning);
                    }
                },
                ActionE::StartFunction(name) => {}
                ActionE::Non => {}
            }
        }
    }
}
