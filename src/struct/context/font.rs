use crate::*;

pub const VECTOR_CHAR_WTIS_CHAR: [char; 5] = ['/', '\\', '.', ':', '|'];

#[derive(Debug, Clone)]
pub struct FontS {
    pub(crate) big: HashMap<char, CharPixel>,
    pub(crate) smal: HashMap<char, CharPixel>,
    pub(crate) number: HashMap<char, CharPixel>,
    pub(crate) char: HashMap<char, CharPixel>,
}

impl Create for FontS {
    type Output = FontS;
    type Facts = ();

    fn new(_facts: Self::Facts) -> Self::Output {
        FontS {
            big: {
                let mut hash_map: HashMap<char, CharPixel> = HashMap::new();

                for r#char in 'A'..='Z' {
                    hash_map.insert(r#char, get_char_pixel(r#char));
                }

                hash_map
            },
            smal: {
                let mut hash_map: HashMap<char, CharPixel> = HashMap::new();

                for r#char in 'a'..='z' {
                    hash_map.insert(r#char, get_char_pixel(r#char));
                }

                hash_map
            },
            number: {
                let mut hash_map: HashMap<char, CharPixel> = HashMap::new();

                for r#char in '0'..='9' {
                    hash_map.insert(r#char, get_char_pixel(r#char));
                }

                hash_map
            },
            char: {
                let mut hash_map: HashMap<char, CharPixel> = HashMap::new();

                for r#char in &VECTOR_CHAR_WTIS_CHAR {
                    hash_map.insert(*r#char, get_char_pixel(*r#char));
                }

                hash_map
            },
        }
    }

    fn default() -> Self::Output {
        return FontS::new(());
    }
}

impl FontS {
    pub fn get_pixel_positions(&self, char_0: char) -> CharPixel {
        return match char_0 {
            'A'..='Z' => {
                if let Some(array_pixel) = self.big.get(&char_0) {
                    array_pixel.clone()
                } else {
                    char_pixel_default()
                }
            }
            'a'..='z' => {
                if let Some(array_pixel) = self.smal.get(&char_0) {
                    array_pixel.clone()
                } else {
                    char_pixel_default()
                }
            }
            '0'..='9' => {
                if let Some(array_pixel) = self.number.get(&char_0) {
                    array_pixel.clone()
                } else {
                    char_pixel_default()
                }
            }
            _ if VECTOR_CHAR_WTIS_CHAR.contains(&char_0) => {
                if let Some(array_pixel) = self.char.get(&char_0) {
                    array_pixel.clone()
                } else {
                    char_pixel_default()
                }
            }
            _ => char_pixel_default(),
        };
    }
}
