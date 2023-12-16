use crate::*;

pub struct Flow;

impl Flow {
    pub fn control(stdout: &mut Stdout) {
        Terminal::on_off(true, true, stdout);

        let mut game = GameS::new(&GameS::give_date());

        let mut last_output_time_render = Instant::now();
        let mut last_output_time_update = Instant::now();

        loop {
            let now_render = Instant::now();
            let elapsed_render = now_render.duration_since(last_output_time_render);

            if elapsed_render >= Duration::from_millis(1000) {
                #[cfg(feature = "render")]
                Flow::render(&mut game, stdout);

                last_output_time_render = now_render;
            }

            let now_update = Instant::now();
            let elapsed_update = now_update.duration_since(last_output_time_update);

            if elapsed_update >= Duration::from_millis(1000) {
                #[cfg(feature = "render")]
                game.gui.update(&game.logic);

                last_output_time_update = now_update;
            }

            if event::poll(Duration::from_millis(250)).error() {
                Flow::event_check(&mut game, stdout);
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
                    Terminal::on_off(false, true, stdout);

                    process::exit(0);
                }
                #[cfg(feature = "gui")]
                KeyCode::Tab => unsafe {
                    game.gui.gui_render.gui_up(&false);
                },
                _ => {}
            }

            #[cfg(feature = "gui")]
            let gui = game.gui.clone();

            #[cfg(feature = "gui")]
            if let Some(path) = PathS::vector_give(&game.gui.path, code) {
                match &path.name {

                    Ok(name_0) => {
                        #[cfg(feature = "button")]
                        if let Some(button) = ButtonS::vector_give(&gui.button, name_0.clone()) {
                            if button.button == Some(code) {
                                ActionE::check_action(game, &button.action);
                            }
                        }
                    }
                    Err(name_1) => {
                        #[cfg(feature = "window")]
                        if let Some(window) = WindowS::vector_give(&gui.window, name_1[0].clone()) {
                            #[cfg(feature = "button")]
                            if let Some(button) =
                                ButtonS::vector_give(&window.button, name_1[1].clone())
                            {
                                if button.button == Some(code) {
                                    ActionE::check_action(game, &button.action);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "button")]
impl ActionE {
    pub fn check_action(game: &mut GameS, action_vector: &Vec<ActionE>) {
        for action in action_vector {
            println!("!");

            match action {
                #[cfg(feature = "button")]
                ActionE::OpenButton(action) => {
                    match action {
                        GuiActionE::OpenAllClose(meaning) => {
                            ButtonS::open_all_close(&mut game.gui.button, &meaning);
                        }
                        GuiActionE::OpenRestClose(name, meaning) => {
                            ButtonS::open_rest_close(&mut game.gui.button, &name, &meaning);
                        }
                        GuiActionE::OpenOneClose(name, meaning) => {
                            ButtonS::open_one_close(&mut game.gui.button, &name, &meaning);
                        }
                    }

                    game.gui.gui_render.button_up(&false);
                },
                #[cfg(feature = "window")]
                ActionE::OpenWindow(action) => {
                    match action {
                        GuiActionE::OpenAllClose(meaning) => {
                            WindowS::open_all_close(&mut game.gui.window, &meaning);
                        }
                        GuiActionE::OpenRestClose(name, meaning) => {
                            WindowS::open_rest_close(&mut game.gui.window, &name, &meaning);
                        }
                        GuiActionE::OpenOneClose(name, meaning) => {
                            WindowS::open_one_close(&mut game.gui.window, &name, &meaning);
                        }
                    }

                    game.gui.gui_render.window_up(&false);
                },
                #[cfg(feature = "table")]
                ActionE::OpenTable(action) => {
                    match action {
                        GuiActionE::OpenAllClose(meaning) => {
                            TableS::open_all_close(&mut game.gui.table, &meaning);
                        }
                        GuiActionE::OpenRestClose(name, meaning) => {
                            TableS::open_rest_close(&mut game.gui.table, &name, &meaning);
                        }
                        GuiActionE::OpenOneClose(name, meaning) => {
                            TableS::open_one_close(&mut game.gui.table, &name, &meaning);
                        }
                    }

                    game.gui.gui_render.table_up(&false);
                }
                ActionE::StartFunction(name) => {}
                _ => {}
            }
        }
    }
}
