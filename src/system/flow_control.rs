use crate::*;

pub struct Flow;

impl Flow {
    pub fn control(stdout: &mut Stdout) {
        let mut game = GameS::new(&GameS::give_date());

        Terminal::on_off(true, true, stdout);

        let mut last_output_time_render = Instant::now();
        let mut last_output_time_update = Instant::now();

        Render::resourse(stdout);

        loop {
            {
                let now_render = Instant::now();
                let elapsed_render = now_render.duration_since(last_output_time_render);

                let last_output_time = Instant::now();

                if elapsed_render >= Duration::from_millis(1000) {
                    Render::resourse_update(&game.logic, stdout);

                    last_output_time_render = now_render;
                }

                {
                    Terminal::teleport_mouse(&[0, 5], stdout);
                    let elapsed_time = Instant::now().duration_since(last_output_time);
                    println!("{:?}", elapsed_time.as_secs_f64());
                }
            }
            {
                let now_update = Instant::now();
                let elapsed_update = now_update.duration_since(last_output_time_update);

                if elapsed_update >= Duration::from_millis(1000) {
                    game.update(&());

                    last_output_time_update = now_update;
                }

                if event::poll(Duration::from_millis(1000)).error() {
                    Flow::event_check(&mut game.logic, stdout);
                }
            }
        }
    }

    pub fn event_check(logic: &mut LogicS, stdout: &mut Stdout) {
       let log = logic.clone();
        
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Esc => {
                    Terminal::on_off(false, true, stdout);
                    process::exit(0);
                },
                _ => {},
            }

            if let Some(country) =
                CountryS::hashmap_give_mut(&mut logic.countries, &logic.name_country[1], true)
            {
                match code {
                    KeyCode::Char('u') => StorageS::build(country, &log, 0, 0),
                    KeyCode::Char('i') => StorageS::build(country, &log, 0, 1),
                    KeyCode::Char('o') => StorageS::build(country, &log, 0, 2),
                    KeyCode::Char('p') => StorageS::build(country, &log, 0, 3),

                    KeyCode::Char('h') => StorageS::destroy(country, &log, 0, 0),
                    KeyCode::Char('j') => StorageS::destroy(country, &log, 0, 1),
                    KeyCode::Char('k') => StorageS::destroy(country, &log, 0, 2),
                    KeyCode::Char('l') => StorageS::destroy(country, &log, 0, 3),
                    _ => {}
                }
            }
        }
    }
}
