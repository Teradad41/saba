use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<FontSize>,
    text_decoration: Option<TextDecoration>,
    height: Option<f64>,
    width: Option<f64>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            display: None,
            font_size: None,
            text_decoration: None,
            height: None,
            width: None,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = Some(color)
    }

    pub fn background_color(&self) -> Color {
        self.background_color
            .clone()
            .expect("failed to access CSS property: background_color")
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color)
    }

    pub fn color(&self) -> Color {
        self.color
            .clone()
            .expect("failed to access CSS property: color")
    }

    pub fn set_display(&mut self, display: DisplayType) {
        self.display = Some(display)
    }

    pub fn display(&self) -> DisplayType {
        self.display
            .clone()
            .expect("failed to access CSS property: display")
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size
            .expect("failed to access CSS property: font_size")
    }

    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .expect("failed to access CSS property: text_decoration")
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = Some(height)
    }

    pub fn height(&self) -> f64 {
        self.height.expect("failed to access CSS property: height")
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = Some(width);
    }

    pub fn width(&self) -> f64 {
        self.width.expect("failed to access CSS property: width")
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Color {
//     name: Option<String>,
//     code: String,
// }

// impl Color {
//     pub fn from_name(name: &str) -> Result<Self, Error> {
//         let code = match name {
//             "black" => "#000000".to_string(),
//         };

//         Ok(Self {
//             name: Some(name.to_string(), code),
//         })
//     }
// }
