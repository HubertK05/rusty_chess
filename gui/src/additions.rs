use eframe::epaint::RectShape;
use egui::{Ui, layers::ShapeIdx, Shape, Color32};

pub fn new_bg(ui: &mut Ui) -> ShapeIdx {
    ui.painter().add(Shape::Noop)
}

pub fn paint_min_rect(ui: &mut Ui, bg: ShapeIdx, color: Color32) {
    ui.painter().set(
        bg,
        RectShape {
            rounding: ui.visuals().widgets.inactive.rounding,
            fill: color,
            stroke: ui.visuals().widgets.inactive.bg_stroke,
            rect: ui.min_rect(),
        },
    );
}

pub fn paint_max_rect(ui: &mut Ui, bg: ShapeIdx, color: Color32) {
    ui.painter().set(
        bg,
        RectShape {
            rounding: ui.visuals().widgets.inactive.rounding,
            fill: color,
            stroke: ui.visuals().widgets.inactive.bg_stroke,
            rect: ui.max_rect(),
        },
    );
}
