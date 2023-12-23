use crate::*;

pub const COLORS: [Color; 16] = [
    Color::White,
    Color::Black,
    Color::Grey,
    Color::DarkGrey,
    Color::Red,
    Color::DarkRed,
    Color::Green,
    Color::DarkGreen,
    Color::Yellow,
    Color::DarkYellow,
    Color::Blue,
    Color::DarkBlue,
    Color::Magenta,
    Color::DarkMagenta,
    Color::Cyan,
    Color::DarkCyan,
];

pub struct ColorR;

impl ColorR {
    pub fn check_2_color(color: &[Option<Color>; 2]) -> [Color; 2] {
        let mut color_array = [Color::White, Color::Black];

        if let Some(color) = color[0] {
            color_array[0] = color;
        }

        if let Some(color) = color[1] {
            color_array[1] = color;
        }

        color_array
    }

    pub fn defaulf_2() -> [Color; 2] {
        [Color::White, Color::Black]
    }
}

//

#[cfg(feature = "render")]
pub struct Render;

#[cfg(feature = "render")]
type Object = Vec<Vec<(char, [Color; 2],)>>;

#[cfg(feature = "render")]
impl Render {
    pub fn draw_object(object: &Object, posit: &[u16; 2], stdout: &mut Stdout) {
        Terminal::teleport_mouse(posit, stdout);

        for y in 0..object.len() {
            for x in 0..object[y].len() {
                let symbol = object[y][x];

                Render::draw_char(&symbol.0, &[Some(symbol.1[0]), Some(symbol.1[1])], stdout);
            }

            Terminal::shift_mouse(&[Err(object[y].len().try_into().error()), Ok(1)], stdout);
        }
    }

    pub fn draw_text(
        text: &Result<String, String>,
        color: &[Option<Color>; 2],
        stdout: &mut Stdout,
    ) {
        let color = ColorR::check_2_color(color);

        execute!(stdout, SetForegroundColor(color[0])).error();
        execute!(stdout, SetBackgroundColor(color[1])).error();

        match text {
            Ok(text) => queue!(stdout, Print(text.to_owned() + "\n")).error(),
            Err(text) => queue!(stdout, Print(text)).error(),
        }

        execute!(stdout, ResetColor).error();
    }

    pub fn draw_char(r#char: &char, color: &[Option<Color>; 2], stdout: &mut Stdout) {
        Render::draw_text(&Err(format!("{}", r#char)), color, stdout);
    }
}

#[cfg(feature = "render")]
impl Render {
    pub fn enter(color: &[Option<Color>; 2], number: u16, stdout: &mut Stdout) {
        let color = ColorR::check_2_color(color);

        execute!(stdout, SetForegroundColor(color[0])).error();
        execute!(stdout, SetBackgroundColor(color[1])).error();

        queue!(stdout, Print("\n".repeat(number as usize))).error();

        execute!(stdout, ResetColor).error();
    }

    pub fn space(color: &[Option<Color>; 2], number: u16, stdout: &mut Stdout) {
        let color = ColorR::check_2_color(color);

        execute!(stdout, SetForegroundColor(color[0])).error();
        execute!(stdout, SetBackgroundColor(color[1])).error();

        queue!(stdout, Print(" ".repeat(number as usize))).error();

        execute!(stdout, ResetColor).error();
    }
}

//

#[cfg(feature = "gui")]
impl GuiS {
    pub fn render(&mut self, stdout: &mut Stdout) {
        let gr = &mut self.gui_render;

        if let Ok(window) = gr.window {
            if let Ok(button) = gr.button {
                if let Ok(table) = gr.table {
                    if !window && !button && !table || !gr.gui {
                        Terminal::clean(stdout);
                    }
                }
            }
        }

        #[cfg(feature = "window")]
        if let Ok(window_res) = gr.window {
            if !window_res || !gr.gui {
                for window in &self.window {
                    if window.draw {
                        WindowS::render(&window, stdout);
                    }
                }
            }

            gr.window_ok(&true); 
        } else if let Err(window_res) = &gr.window {
            for name in window_res {
                if let Ok(name) = name {
                    if let Some(window) = WindowS::vector_give(&self.window, &name) {
                        if window.draw {
                            WindowS::render(&window, stdout);
                        }
                    }
                } else if let Err(name) = name {
                    if let Some(window) = WindowS::vector_give(&self.window, &name[0]) {
                        if let Some(button) = ButtonS::vector_give(&window.button, &name[1]) {
                            if button.draw {
                                ButtonS::render(&button, &Some(window.aabb), stdout);
                            }
                        }
                    }
                }
            }

            gr.window_ok(&true);        
        }
        

        #[cfg(feature = "button")]
        if let Ok(button_res) = gr.button {
            if !button_res || !gr.gui {
                for button in &self.button {
                    if button.draw {
                        ButtonS::render(&button, &None, stdout);
                    }
                }
            }

            gr.button_ok(&true);
        } else if let Err(button_res) = &gr.button {
            for name in button_res {
                if let Some(button) = ButtonS::vector_give(&self.button, name) {
                    if button.draw {
                        ButtonS::render(&button, &None, stdout);
                    }
                }
            }

            gr.button_ok(&true);
        }

        

        #[cfg(feature = "table")]
        if let Ok(table_res) = gr.table {
            if !table_res || !gr.gui {
                for table in &self.table {
                    if table.draw {
                        TableS::render(&table, true, stdout);
                    }
                }
            }

            gr.table_ok(&true);
        } else if let Err(table_res) = &gr.table {
            for name in table_res {
                if let Some(table) = TableS::vector_give(&self.table, &name.0) {
                    if table.draw {
                        TableS::render(&table, name.1, stdout);
                    }
                }
            }

            gr.table_ok(&true);
        }

        if let Ok(window) = gr.window {
            if let Ok(button) = gr.button {
                if let Ok(table) = gr.table {
                    if window && button && table {
                        gr.gui_update(&true);
                    }
                }
            }
        }
    }

    pub fn draw_border(aabb_0: &AabbS, color: &[Color; 2], draw: &[bool; 2], stdout: &mut Stdout) {
        let [x, y] = [aabb_0.position.x, aabb_0.position.y];
        let [width, height] = [aabb_0.size.width, aabb_0.size.height];

        if draw[0] {
            let color_array = [Some(color[0]), Some(color[0])];
            {
                Terminal::teleport_mouse(&[x, y], stdout);
                Render::space(&color_array, width, stdout);

                Terminal::teleport_mouse(&[x, y + height - 1], stdout);
                Render::space(&color_array, width, stdout);
            }
            {
                for i in (y)..(y + height) {
                    Terminal::teleport_mouse(&[x, i], stdout);
                    Render::space(&color_array, 1, stdout);
                }

                for i in (y)..(y + height) {
                    Terminal::teleport_mouse(&[x + width - 1, i], stdout);
                    Render::space(&color_array, 1, stdout);
                }
            }
        }

        if draw[1] {
            let color_array = [Some(color[1]), Some(color[1])];
            if draw[0] {
                Terminal::teleport_mouse(&[x + 1, y + 1], stdout);

                for q in (y + 1)..(y + height - 1) {
                    Terminal::teleport_mouse(&[x + 1, q], stdout);
                    Render::space(&color_array, width - 2, stdout);
                }
            } else {
                Terminal::teleport_mouse(&[x, y], stdout);

                for q in (y)..(y + height) {
                    Terminal::teleport_mouse(&[x, q], stdout);
                    Render::space(&color_array, width, stdout);
                }
            }
        }
    }

    pub fn draw_text(text: &String, color: &[Color; 2], aabb: &AabbS, stdout: &mut Stdout) {
        Terminal::teleport_mouse(&[aabb.position.x + 1, aabb.position.y + 1], stdout);

        let mut vec_char = vec![Vec::new()]; //text.chars().collect();

        let mut num = 0;

        for ch in text.chars() {
            if ch == '\n' {
                vec_char.push(Vec::new());

                num += 1;
            } else {
                vec_char[num].push((ch, color.clone()));
            }
        }

        Render::draw_object(
            &vec_char,
            &[aabb.position.x + 1, aabb.position.y + 1],
            stdout,
        );
    }
}

#[cfg(feature = "button")]
impl ButtonS {
    pub fn render(&self, aabb_0: &Option<AabbS>, stdout: &mut Stdout) {
        let mut aabb = self.aabb;

        if let Some(aabb_0) = aabb_0 {
            aabb = AabbS::aabb_add_aabb(&aabb_0, &self.aabb);
        }

        Terminal::teleport_mouse(&[aabb.position.x, aabb.position.y], stdout);

        #[cfg(feature = "gui")]
        GuiS::draw_border(
            &aabb,
            &self.color,
            &[self.flooded_border, self.interior],
            stdout,
        );

        if let Some(text) = &self.text {
            #[cfg(feature = "gui")]
            GuiS::draw_text(&text.1, &text.0, &aabb, stdout);
        }
    }
}

#[cfg(feature = "window")]
impl WindowS {
    pub fn render(&self, stdout: &mut Stdout) {
        Terminal::teleport_mouse(&[self.aabb.position.x, self.aabb.position.y], stdout);

        #[cfg(feature = "gui")]
        GuiS::draw_border(
            &self.aabb,
            &self.color,
            &[self.flooded_border, self.interior],
            stdout,
        );

        #[cfg(feature = "button")]
        for button in &self.button {
            if button.draw {
                ButtonS::render(&button, &Some(self.aabb), stdout);
            }
        }
    }
}

#[cfg(feature = "table")]
impl TableS {
    pub fn render(&self, border: bool, stdout: &mut Stdout) {
        let num_y = if self.cells.len() > 0 {
            self.cells.len()
        } else {
            return;
        };

        let num_x = if num_y != 0 { self.cells[0].1.len() } else { 0 };

        for q in 0..num_y {
            let mut aabb = AabbS::new(&(
                [
                    self.position.x + q as u16 * self.cells[q].0.position.x,
                    self.position.y + q as u16 * self.cells[q].0.position.y,
                ],
                [self.cells[q].0.size.width, self.cells[q].0.size.height],
            ));

            for w in 0..num_x {
                if w > 0 {
                    PositionS::add(
                        &mut aabb.position,
                        &[Ok(0), Ok(self.cells[q].0.size.height + self.indentation)],
                    );
                }

                #[cfg(feature = "gui")]
                if border {
                    GuiS::draw_border(
                        &aabb,
                        &self.color,
                        &[self.flooded_border, self.interior],
                        stdout,
                    );
                }

                #[cfg(feature = "gui")]
                if let Some(text) = self.cells[q].1.get(w) {
                    GuiS::draw_text(&text.1, &text.0, &aabb, stdout);
                }
            }
        }
    }
}
