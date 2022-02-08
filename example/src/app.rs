use eframe::{egui, egui::*, epi};

pub struct SvgApp {
    svg: egui_svg::EguiSvg,
}

impl Default for SvgApp {
    fn default() -> Self {
        SvgApp {
            svg: egui_svg::EguiSvg::default(),
        }
    }
}

impl epi::App for SvgApp {
    fn name(&self) -> &str {
        "egui_svg_example"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.svg.ui(ui);
        });
    }
}
