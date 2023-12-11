use crate::*;

pub struct Terminal;

impl Terminal {
    pub fn shift_mouse(posit: &[Result<u16, u16>; 2], stdout: &mut Stdout) {
        if let Ok((x_cursor, y_cursor)) = cursor::position() {
            match posit[0] {
                Ok(x) => queue!(stdout, MoveRight(x)).error(),
                Err(x) => {
                    if x_cursor > 1 {
                        queue!(stdout, MoveLeft(x)).error();
                    }
                }
            }

            match posit[1] {
                Ok(y) => queue!(stdout, MoveDown(y)).error(),
                Err(y) => {
                    if y_cursor > 1 {
                        queue!(stdout, MoveUp(y)).error();
                    }
                }
            }
        }

        stdout.flush().error();
    }

    pub fn shift_mouse_05(posit: &[Result<u16, u16>; 2], stdout: &mut Stdout) {
        unsafe {
            pub static mut SH: u8 = 0;

            if SH == 1 {
                Self::shift_mouse(posit, stdout);

                SH = 0;
            } else {
                SH += 1;
            }
        }
    }

    //

    pub fn teleport_mouse(posit: &[u16; 2], stdout: &mut Stdout) {
        queue!(stdout, MoveTo(posit[0], posit[1])).error();

        stdout.flush().error();
    }

    pub fn teleport_mouse_05(posit: &[u16; 2], stdout: &mut Stdout) {
        unsafe {
            pub static mut TP: u8 = 0;

            if TP == 1 {
                Self::teleport_mouse(posit, stdout);

                TP = 0;
            } else {
                TP += 1;
            }
        }
    }
}

impl Terminal {
    pub fn hide_show(r#bool: bool, stdout: &mut Stdout) {
        match r#bool {
            true => {
                queue!(stdout, Hide).error();
            }
            false => {
                queue!(stdout, Show).error();
            }
        }

        stdout.flush().error();
    }

    pub fn raw_mode(r#bool: bool, stdout: &mut Stdout) {
        match r#bool {
            true => {
                enable_raw_mode().error();
                execute!(stdout, EnterAlternateScreen).error();
            }
            false => {
                execute!(stdout, LeaveAlternateScreen).error();
                disable_raw_mode().error();
            }
        }
    }

    pub fn set_size() {
        let os = env::consts::OS;

        match os {
            "windows" => {
                Command::new("cmd")
                    .args(&["/C", "mode con: cols=250 lines=150"])
                    .spawn()
                    .expect("Failed to execute command");
            }
            "linux" | "macos" => {
                Command::new("sh")
                    .arg("-c")
                    .arg("printf '\\e[8;250;150t'")
                    .spawn()
                    .expect("Failed to execute command");
            }
            _ => {
                println!("Unsupported OS: {}", os);
            }
        }
    }

    pub fn on_off(hide_show: bool, raw_mode: bool, stdout: &mut Stdout) {
        Terminal::clean(stdout);
        Terminal::set_size();
        Terminal::hide_show(hide_show, stdout);
        Terminal::raw_mode(raw_mode, stdout);
    }
}

impl Terminal {
    pub fn clean(stdout: &mut Stdout) {
        execute!(stdout, Clear(ClearType::All)).error();
    }
}
