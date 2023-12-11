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

pub struct Render;

type Object = Vec<Vec<(char, [Color; 2])>>;

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
            Ok(text) => println!("{}", text),
            Err(text) => print!("{}", text),
        }

        execute!(stdout, ResetColor).error();
    }

    pub fn draw_char(r#char: &char, color: &[Option<Color>; 2], stdout: &mut Stdout) {
        Render::draw_text(&Err(format!("{}", r#char)), color, stdout);
    }
}

impl Render {
    pub fn enter(color: &[Option<Color>; 2], number: usize, stdout: &mut Stdout) {
        let color = ColorR::check_2_color(color);

        execute!(stdout, SetForegroundColor(color[0])).error();
        execute!(stdout, SetBackgroundColor(color[1])).error();

        for _ in 0..number {
            print!("\n");
        }

        execute!(stdout, ResetColor).error();
    }

    pub fn space(color: &[Option<Color>; 2], number: usize, stdout: &mut Stdout) {
        let color = ColorR::check_2_color(color);

        execute!(stdout, SetForegroundColor(color[0])).error();
        execute!(stdout, SetBackgroundColor(color[1])).error();

        for _ in 0..number {
            print!(" ");
        }

        execute!(stdout, ResetColor).error();
    }
}

//

impl GuiS {
    pub fn render(&self, stdout: &mut Stdout) {
        for button in &self.button {
            if button.draw {
                ButtonS::render(&button, &None, stdout);
            }
        }

        for window in &self.window {
            if window.draw {
                WindowS::render(&window, stdout);
            }
        }

        for table in &self.table {
            if table.draw {
                TableS::render(&table, stdout);
            }
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
                for i in (x + 1)..(x + width - 1) {
                    Terminal::teleport_mouse(&[i, y], stdout);
                    Render::draw_char(&'-', &color_array, stdout);

                    Terminal::teleport_mouse(&[i, y + height - 1], stdout);
                    Render::draw_char(&'-', &color_array, stdout);
                }
                for i in (y + 1)..(y + height - 1) {
                    Terminal::teleport_mouse(&[x, i], stdout);
                    Render::draw_char(&'|', &color_array, stdout);

                    Terminal::teleport_mouse(&[x + width - 1, i], stdout);
                    Render::draw_char(&'|', &color_array, stdout);
                }
            }
        }
        if let Some(flooded_border) = flooded_border {
            for q in (y + 1)..(y + height - 1) {
                for w in (x + 1)..(x + width - 1) {
                    Terminal::teleport_mouse(&[w, q], stdout);
                    Render::draw_char(&'0', &[Some(color[1]), Some(color[1])], stdout);
                }
            }
        } else {
            for q in (y)..(y + height) {
                for w in (x)..(x + width) {
                    Terminal::teleport_mouse(&[w, q], stdout);
                    Render::draw_char(&'0', &[Some(color[1]), Some(color[1])], stdout);
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

impl ButtonS {
    pub fn render(&self, aabb_0: &Option<AabbS>, stdout: &mut Stdout) {
        let mut aabb = self.aabb;

        if let Some(aabb_0) = aabb_0 {
            aabb = AabbS::aabb_add_aabb(&aabb_0, &self.aabb);
        }

        Terminal::teleport_mouse(&[aabb.position.x, aabb.position.y], stdout);

        GuiS::draw_border(&aabb, &self.color, self.flooded_border, stdout);

        if let Some(text) = &self.text {
            GuiS::draw_text(&text.0, &text.1, &aabb, stdout);
        }
    }
}

impl WindowS {
    pub fn render(&self, stdout: &mut Stdout) {
        Terminal::teleport_mouse(&[self.aabb.position.x, self.aabb.position.y], stdout);

        GuiS::draw_border(&self.aabb, &self.color, self.flooded_border, stdout);

        for button in &self.button {
            if button.draw {
                ButtonS::render(&button, &Some(self.aabb), stdout);
            }
        }
    }
}

impl TableS {
    pub fn render(&self, stdout: &mut Stdout) {
        let num_y = self.cells.len();

        let num_x = if num_y != 0 { self.cells[0].len() } else { 0 };

        for q in 0..num_y {
            for w in 0..num_x {
                let aabb = AabbS::new(&(
                    [
                        self.aabb.position.x + w as u16 * self.aabb.size.width + w as u16 * self.indentation,
                        self.aabb.position.y + q as u16 * self.aabb.size.height + q as u16 * self.indentation,
                    ],
                    [self.aabb.size.width, self.aabb.size.height],
                ));

                GuiS::draw_border(&aabb, &self.color, self.flooded_border, stdout);

                if let Some(text) = self.cells[q].get(w) {
                    GuiS::draw_text(&text.0, &text.1, &aabb, stdout);
                }
            }
        }
    }
}
