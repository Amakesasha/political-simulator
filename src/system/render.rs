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

#[cfg(feature = "render")]
impl Flow {
    pub fn render(game: &mut GameS, stdout: &mut Stdout) {
        #[cfg(feature = "gui")]
        GuiS::render(&mut game.gui, stdout);
    }
}

#[cfg(feature = "render")]
pub struct Render;

#[cfg(feature = "render")]
type Object = Vec<Vec<(char, [Color; 2], )>>;

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

        if !gr.gui || !gr.gui && !gr.window && !gr.table {
            Terminal::clean(stdout);
        }

        #[cfg(feature = "button")]
        if !gr.button || !gr.gui {
            for button in &self.button {
                if button.draw {
                    #[cfg(feature = "button")]
                    ButtonS::render(&button, &None, stdout);
                }
            }

            gr.button = true;
        }

        #[cfg(feature = "window")]
        if !gr.window || !gr.gui {
            for window in &self.window {
                if window.draw {
                    #[cfg(feature = "window")]
                    WindowS::render(&window, stdout);
                }
            }

            gr.window = true;
        }

        #[cfg(feature = "table")]
        if !gr.table || !gr.gui {
            for table in &self.table {
                if table.draw {
                    #[cfg(feature = "table")]
                    TableS::render(&table, stdout);
                }
            }

            gr.table = true;
        }

        if gr.window && gr.button && gr.table {
            gr.gui = true;
        }
    }

    pub fn udate_render(&mut self, stdout: &mut Stdout) {
        let gr = &mut self.gui_render;

        if !gr.gui || !gr.gui && !gr.window && !gr.table {
            Terminal::clean(stdout);
        }

        #[cfg(feature = "button")]
        if !gr.button || !gr.gui {
            for button in &self.button {
                if button.draw {
                    #[cfg(feature = "button")]
                    ButtonS::render(&button, &None, stdout);
                }
            }

            gr.button = true;
        }

        #[cfg(feature = "window")]
        if !gr.window || !gr.gui {
            for window in &self.window {
                if window.draw {
                    #[cfg(feature = "window")]
                    WindowS::render(&window, stdout);
                }
            }

            gr.window = true;
        }

        #[cfg(feature = "table")]
        if !gr.table || !gr.gui {
            for table in &self.table {
                if table.draw {
                    #[cfg(feature = "table")]
                    TableS::render(&table, stdout);
                }
            }

            gr.table = true;
        }

        if gr.window && gr.button && gr.table {
            gr.gui = true;
        }
    }

    pub fn draw_border(
        aabb_0: &AabbS,
        color: &[Color; 2],
        flooded_border: Option<bool>,
        stdout: &mut Stdout,
    ) {
        let [x, y] = [aabb_0.position.x, aabb_0.position.y];
        let [width, height] = [aabb_0.size.width, aabb_0.size.height];

        if let Some(flooded_border) = flooded_border {
            let color_array = [
                Some(color[0]),
                Some(color[if flooded_border { 0 } else { 1 }]),
            ];

            {
                let position = [
                    [x, y],
                    [x, y + height - 1],
                    [x + width - 1, y + height - 1],
                    [x + width - 1, y],
                ];
                for posit in &position {
                    Terminal::teleport_mouse(&posit, stdout);

                    Render::draw_char(&'+', &color_array, stdout)
                }
            }
            {
                Terminal::teleport_mouse(&[x, y], stdout);
                Render::space(&color_array, width, stdout);

                Terminal::teleport_mouse(&[x, y + height - 1], stdout);
                Render::space(&color_array, width, stdout);
            }
            {
                for i in (y + 1)..(y + height - 1) {
                    Terminal::teleport_mouse(&[x, i], stdout);
                    Render::draw_char(&'|', &color_array, stdout);

                    Terminal::teleport_mouse(&[x + width - 1, i], stdout);
                    Render::draw_char(&'|', &color_array, stdout);
                }
            }
        }

        if let Some(_) = flooded_border {
            Terminal::teleport_mouse(&[x + 1, y + 1], stdout);

            for q in (y + 1)..(y + height) {
                for _ in (x + 1)..(x + width - 1) {
                    Render::draw_char(&'0', &[Some(color[1]), Some(color[1])], stdout);
                }

                Terminal::teleport_mouse(&[x + 1, q], stdout);
            }
        } else {
            Terminal::teleport_mouse(&[x, y], stdout);

            for q in (y)..(y + height) {
                for _ in (x)..(x + width) {
                    Render::draw_char(&'0', &[Some(color[1]), Some(color[1])], stdout);
                }

                Terminal::teleport_mouse(&[x, q], stdout);
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
        GuiS::draw_border(&aabb, &self.color, self.flooded_border, stdout);

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
        GuiS::draw_border(&self.aabb, &self.color, self.flooded_border, stdout);

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
    pub fn render(&self, stdout: &mut Stdout) {
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
                GuiS::draw_border(&aabb, &self.color, self.flooded_border, stdout);

                if let Some(text) = self.cells[q].1.get(w) {
                    #[cfg(feature = "gui")]
                    GuiS::draw_text(&text.1, &text.0, &aabb, stdout);
                }
            }
        }
    }
}
