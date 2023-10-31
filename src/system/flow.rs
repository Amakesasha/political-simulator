use crate::*;

impl GameS {
    pub fn flow() {
        let mut display: PistonWindow = WindowSettings::new("Game", [800, 600])
            .exit_on_esc(false)
            .decorated(true)
            .build()
            .unwrap();

        let mut game = GameS::new(GameS::give_date());

        //display.window.window.set_maximized(true);

        let mut event = loop {
            if let Some(event_new) = display.next() {
                break event_new;
            }
        };

        let mut i = 0;

        while game.context.play_stop[0] {
            {
                if let Some(event_new) = display.next() {
                    event = event_new;
                } else {
                    return;
                }
            }

            {
                std::thread::sleep(std::time::Duration::from_secs_f64(0.05));

                if !game.context.play_stop[1] {
                    #[cfg(feature = "event")]
                    game.event(&mut display, &event);

                    if i >= 10 {
                        game.update(());

                        i = 0;
                    } else {
                        i += 1;
                    }

                    #[cfg(feature = "draw")]
                    game.draw(&mut display, &event);
                }
            }
        }
    }
}
