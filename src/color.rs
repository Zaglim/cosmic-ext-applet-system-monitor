use cosmic::{
    cosmic_theme::palette::{encoding::srgb::Srgb, rgb::Rgb, rgb::Rgba, Srgba},
    Theme,
};
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub enum Color {
    gray_1,
    gray_2,
    neutral_0,
    neutral_1,
    neutral_2,
    neutral_3,
    neutral_4,
    neutral_5,
    neutral_6,
    neutral_7,
    neutral_8,
    neutral_9,
    neutral_10,
    bright_green,
    bright_red,
    bright_orange,
    ext_warm_grey,
    ext_orange,
    ext_yellow,
    ext_blue,
    ext_purple,
    ext_pink,
    ext_indigo,
    accent_blue,
    accent_red,
    accent_green,
    accent_warm_grey,
    accent_orange,
    accent_yellow,
    accent_purple,
    accent_pink,
    accent_indigo,
    rgb(String),
}

impl Color {
    pub fn as_rgb_color(&self, theme: &Theme) -> RGBColor {
        let accent_color = theme.cosmic().accent_color();
        let palette = &theme.cosmic().palette;
        color_to_rgb(self.as_srgba(theme).color)
    }

    pub fn as_srgba(&self, theme: &Theme) -> Srgba {
        let accent_color = theme.cosmic().accent_color();
        let palette = &theme.cosmic().palette;
        match self {
            Color::gray_1 => palette.gray_1,
            Color::gray_2 => palette.gray_2,
            Color::neutral_0 => palette.neutral_0,
            Color::neutral_1 => palette.neutral_1,
            Color::neutral_2 => palette.neutral_2,
            Color::neutral_3 => palette.neutral_3,
            Color::neutral_4 => palette.neutral_4,
            Color::neutral_5 => palette.neutral_5,
            Color::neutral_6 => palette.neutral_6,
            Color::neutral_7 => palette.neutral_7,
            Color::neutral_8 => palette.neutral_8,
            Color::neutral_9 => palette.neutral_9,
            Color::neutral_10 => palette.neutral_10,
            Color::bright_green => palette.bright_green,
            Color::bright_red => palette.bright_red,
            Color::bright_orange => palette.bright_orange,
            Color::ext_warm_grey => palette.ext_warm_grey,
            Color::ext_orange => palette.ext_orange,
            Color::ext_yellow => palette.ext_yellow,
            Color::ext_blue => palette.ext_blue,
            Color::ext_purple => palette.ext_purple,
            Color::ext_pink => palette.ext_pink,
            Color::ext_indigo => palette.ext_indigo,
            Color::accent_blue => palette.accent_blue,
            Color::accent_red => palette.accent_red,
            Color::accent_green => palette.accent_green,
            Color::accent_warm_grey => palette.accent_warm_grey,
            Color::accent_orange => palette.accent_orange,
            Color::accent_yellow => palette.accent_yellow,
            Color::accent_purple => palette.accent_purple,
            Color::accent_pink => palette.accent_pink,
            Color::accent_indigo => palette.accent_indigo,
            Color::rgb(s) => s
                .parse::<Rgba<Srgb, u8>>()
                .map(Rgba::into_format)
                .unwrap_or(accent_color),
        }
    }
}

pub fn color_to_rgb(color: Rgb<Srgb, f32>) -> RGBColor {
    let rgb = color.into_format::<u8>();
    RGBColor(rgb.red, rgb.green, rgb.blue)
}
