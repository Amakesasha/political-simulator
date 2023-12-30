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
            Ok(text) => queue!(stdout, Print(text.to_owned() + "\n")).error(),
            Err(text) => queue!(stdout, Print(text)).error(),
        }

        execute!(stdout, ResetColor).error();
    }

    pub fn draw_char(r#char: &char, color: &[Option<Color>; 2], stdout: &mut Stdout) {
        Render::draw_text(&Err(format!("{}", r#char)), color, stdout);
    }
}

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

struct Border;

impl Border {
    pub fn border(aabb: [[u16; 2]; 2], color: &[Option<Color>; 2], stdout: &mut Stdout) {
        Border::face(aabb, color, stdout);

        Border::vertical_border(aabb, color, stdout);
        Border::horizontal_border(aabb, color, stdout);
    }

    pub fn face(aabb: [[u16; 2]; 2], color: &[Option<Color>; 2], stdout: &mut Stdout) {
        let q = [
            [aabb[0][0], aabb[0][1]],
            [aabb[0][0], aabb[0][1] + aabb[1][1]],
            [aabb[0][0] + aabb[1][0], aabb[0][1] + aabb[1][1]],
            [aabb[0][0] + aabb[1][0], aabb[0][1]],
        ];

        for i in 0..q.len() {
            Terminal::teleport_mouse(&q[i], stdout);
            Render::draw_char(&'+', color, stdout);
        }
    }

    pub fn vertical_border(aabb: [[u16; 2]; 2], color: &[Option<Color>; 2], stdout: &mut Stdout) {
        for i in (aabb[0][1] + 1)..(aabb[0][1] + aabb[1][1]) {
            Terminal::teleport_mouse(&[aabb[0][0], i], stdout);
            Render::draw_char(&'|', color, stdout);

            Terminal::teleport_mouse(&[aabb[0][0] + aabb[1][0], i], stdout);
            Render::draw_char(&'|', color, stdout);
        }
    }

    pub fn horizontal_border(aabb: [[u16; 2]; 2], color: &[Option<Color>; 2], stdout: &mut Stdout) {
        Terminal::teleport_mouse(&[aabb[0][0] + 1, aabb[0][1]], stdout);
        Render::draw_text(&Err("-".repeat((aabb[1][0] - 1) as usize)), color, stdout);

        Terminal::teleport_mouse(&[aabb[0][0] + 1, aabb[0][1] + aabb[1][1]], stdout);
        Render::draw_text(&Err("-".repeat((aabb[1][0] - 1) as usize)), color, stdout);
    }
}

impl Render {
    pub fn resourse(stdout: &mut Stdout) {
        let [x, y] = [10, 10];
        let size = [50, 24];

        Border::border([[x, y], [49, size[1]]], &[Some(Color::Green); 2], stdout);

        Border::vertical_border([[x, y], [11, size[1]]], &[Some(Color::Green); 2], stdout);

        Border::vertical_border([[x, y], [22, size[1]]], &[Some(Color::Green); 2], stdout);

        Border::vertical_border([[x, y], [36, size[1]]], &[Some(Color::Green); 2], stdout);

        Border::horizontal_border([[x, 14], [size[0], 0]], &[Some(Color::Green); 2], stdout);

        Border::horizontal_border([[x, 18], [size[0], 0]], &[Some(Color::Green); 2], stdout);

        Border::horizontal_border([[x, 22], [size[0], 0]], &[Some(Color::Green); 2], stdout);

        Border::horizontal_border([[x, 26], [size[0], 0]], &[Some(Color::Green); 2], stdout);

        Border::horizontal_border([[x, 30], [size[0], 0]], &[Some(Color::Green); 2], stdout);

        //

        {
            Terminal::teleport_mouse(&[x + 2, y + 2], stdout);
            Render::draw_text(&Err(String::from("Resourse")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 2, y + 6], stdout);
            Render::draw_text(&Err(String::from("Concrete")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 4, y + 10], stdout);
            Render::draw_text(&Err(String::from("Tree")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 4, y + 14], stdout);
            Render::draw_text(&Err(String::from("Iron")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 3, y + 18], stdout);
            Render::draw_text(&Err(String::from("Rubber")), &[None; 2], stdout);
        }

        {
            Terminal::teleport_mouse(&[x + 13, y + 2], stdout);
            Render::draw_text(&Err(String::from("Quantity")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 24, y + 2], stdout);
            Render::draw_text(&Err(String::from("Num Factory")), &[None; 2], stdout);

            Terminal::teleport_mouse(&[x + 38, y + 2], stdout);
            Render::draw_text(&Err(String::from("Production")), &[None; 2], stdout);

            //production
        }
    }

    pub fn resourse_update(logic: &LogicS, stdout: &mut Stdout) {
        let [x, y] = [10, 10];
        let size = [50, 24];

        let c = [None; 2];

        if let Some(country) =
            CountryS::hashmap_give(&logic.countries, &logic.name_country[1], true)
        {
            let stor = &country.storage[0].0;

            {
                let [x, y] = [x + 13, y + 6];

                for i in 0..4 {
                    let q = (i * 4) as u16;

                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::space(&c, 9, stdout);
                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::draw_text(&Err(format!("{}", stor[i].quantity)), &c, stdout);
                }
            }

            {
                let [x, y] = [x + 24, y + 6];

                for i in 0..4 {
                    let q = (i * 4) as u16;

                    let num = match i {
                        0 => ["U", "H"],
                        1 => ["I", "J"],
                        2 => ["O", "K"],
                        3 => ["P", "L"],
                        _ => ["", ""],
                    };

                    let text = format!("{} ({}|{})", stor[i].number_of_factory, num[0], num[1]);

                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::space(&c, 12, stdout);
                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::draw_text(&Err(text), &c, stdout);
                }
            }

            {
                let [x, y] = [x + 38, y + 6];

                for i in 0..4 {
                    let q = (i * 4) as u16;

                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::space(&c, 11, stdout);
                    Terminal::teleport_mouse(&[x, y + q], stdout);
                    Render::draw_text(
                        &Err(format!(
                            "{}",
                            stor[i].number_of_factory as f64 * stor[i].production_1_factory
                        )),
                        &c,
                        stdout,
                    );
                }
            }
        }
    }
}
