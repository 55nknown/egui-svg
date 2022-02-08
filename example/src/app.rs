use eframe::{egui, egui::*, epi};

pub struct SvgApp {}

impl Default for SvgApp {
    fn default() -> Self {
        SvgApp {}
    }
}

impl epi::App for SvgApp {
    fn name(&self) -> &str {
        "egui_svg_example"
    }

    fn update(&mut self, _ctx: &egui::CtxRef, _frame: &epi::Frame) {}
}
