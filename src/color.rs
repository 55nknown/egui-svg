use egui::Color32;
use tiny_skia::PremultipliedColorU8;

pub enum SvgColor {
    Original,
    Color(Color32),
}

impl SvgColor {
    pub fn map(&self, p: &PremultipliedColorU8) -> Color32 {
        match self {
            Self::Original => {
                Color32::from_rgba_premultiplied(p.red(), p.green(), p.blue(), p.alpha())
            }
            Self::Color(c) => Color32::from_rgba_unmultiplied(
                c.r(),
                c.g(),
                c.b(),
                (p.alpha() as f32 * (c.a() as f32 / 255_f32)).round() as u8,
            ),
        }
    }
}
