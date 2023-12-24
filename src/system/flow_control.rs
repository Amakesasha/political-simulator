use crate::*;

pub struct Flow;

impl Flow {
    pub fn control(stdout: &mut Stdout) {
        let mut game = GameS::new(&GameS::give_date());

        Terminal::on_off(true, true, stdout);

        let mut last_output_time_render = Instant::now();
        let mut last_output_time_update = Instant::now();

        loop {
            {
                let now_render = Instant::now();
                let elapsed_render = now_render.duration_since(last_output_time_render);

                let last_output_time = Instant::now();

                if elapsed_render >= Duration::from_millis(1000) {
                    #[cfg(feature = "render")]
                    GuiS::render(&mut game.gui, stdout);

                    last_output_time_render = now_render;
                }

                Terminal::teleport_mouse(&[0, 50], stdout);
                {
                    let elapsed_time = Instant::now().duration_since(last_output_time);
                    /*let one_second = Duration::from_secs(1);
                    let adjusted_time = if elapsed_time >= one_second {
                        elapsed_time - one_second
                    } else {
                        Duration::from_secs(0)
                    };*/
                    println!("{:?}", elapsed_time.as_secs_f64());
                }
            }
            {
                let now_update = Instant::now();
                let elapsed_update = now_update.duration_since(last_output_time_update);

                if elapsed_update >= Duration::from_millis(1000) {
                    #[cfg(feature = "render")]
                    game.update(&());

                    last_output_time_update = now_update;
                }

                if event::poll(Duration::from_millis(1000)).error() {
                    Flow::event_check(&mut game, stdout);
                }
            }
        }
    }

    pub fn event_check(game: &mut GameS, stdout: &mut Stdout) {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Esc => {
                    Terminal::on_off(false, true, stdout);

                    process::exit(0);
                }
                #[cfg(feature = "gui")]
                KeyCode::Tab => {
                    game.gui.gui_render.gui_update(&false);
                }
                _ => {}
            }

            #[cfg(feature = "gui")]
            let gui = game.gui.clone();

            #[cfg(feature = "gui")]
            if let Some(path) = PathS::vector_give(&gui.path, &code) {
                match &path.name {
                    Ok(name_0) =>
                    {
                        #[cfg(feature = "button")]
                        if let Some(button) = ButtonS::vector_give(&gui.button, &name_0) {
                            if button.button == Some(code) {
                                ActionE::check_action(game, &button.action);
                            }
                        }
                    }
                    Err(name_1) => {
                        #[cfg(feature = "window")]
                        if let Some(window) = WindowS::vector_give(&gui.window, &name_1[0]) {
                            #[cfg(feature = "button")]
                            if let Some(button) = ButtonS::vector_give(&window.button, &name_1[1]) {
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
        unsafe {
            static mut STATIC: bool = false;

            if STATIC {
                STATIC = false;
            } else {
                STATIC = true;

                return;
            }
        }

        for action in action_vector {
            match action {
                #[cfg(feature = "window")]
                ActionE::OpenWindow(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        WindowS::open_all_close(&mut game.gui.window, &meaning);
                        game.gui.gui_render.window_ok(&meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        WindowS::open_one_close(&mut game.gui.window, &name, &meaning);
                        game.gui
                            .gui_render
                            .window_err(&meaning, &vec![Ok(name.clone())]);
                    }
                },
                #[cfg(feature = "button")]
                ActionE::OpenButton(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        ButtonS::open_all_close(&mut game.gui.button, &meaning);
                        game.gui.gui_render.button_ok(&meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        ButtonS::open_one_close(&mut game.gui.button, &name, &meaning);
                        game.gui
                            .gui_render
                            .button_err(&meaning, &vec![name.clone()]);
                    }
                },
                #[cfg(feature = "table")]
                ActionE::OpenTable(action) => match action {
                    GuiActionE::OpenAllClose(meaning) => {
                        TableS::open_all_close(&mut game.gui.table, &meaning);
                        game.gui.gui_render.table_ok(&meaning);
                    }
                    GuiActionE::OpenOneClose(name, meaning) => {
                        TableS::open_one_close(&mut game.gui.table, &name, &meaning);
                        game.gui
                            .gui_render
                            .table_err(&meaning, &vec![(name.clone(), true)]);
                    }
                },
                ActionE::StartFunction(_name) => {}
                _ => {}
            }
        }
    }
}
