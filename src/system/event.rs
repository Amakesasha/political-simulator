use crate::*;

impl GameS {
    #[cfg(feature = "event")]
    pub fn event(&mut self, display: &mut PistonWindow, event: &Event) {
        match event {
            Event::Input(ref input, _time) => self.event_input(display, input),
            _ => {}
        }
    }

    #[cfg(feature = "event")]
    pub fn event_input(&mut self, _display: &mut PistonWindow, input: &Input) {
        match input {
            Input::Button(button_args) => match button_args.state {
                ButtonState::Press => self.check_button_press(button_args.button),
                ButtonState::Release => self.check_button_release(button_args.button),
            },
            Input::Move(Motion::MouseCursor(_position)) => {}
            _ => {}
        }
    }
}

impl GameS {
    #[cfg(feature = "event")]
    pub fn check_button_press(&mut self, button: Button) {
        if button == self.context.button.esc {
            self.context.play_stop[0] = false;
        }

        if let Some(country) = CountryS::give_mut(&mut self.logic.countries, "Country".to_string())
        {
            match button {
                _ if button == self.context.button.d1 => country.add_factory_concrete(),
                _ if button == self.context.button.d2 => country.add_factory_tree(),
                _ if button == self.context.button.d3 => country.add_factory_iron(),
                _ => {}
            }
        }
    }

    #[cfg(feature = "event")]
    pub fn check_button_release(&mut self, button: Button) {
        match button {
            _ => {}
        }
    }
}
