extern crate sdl2;

use self::sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

use crate::model;

pub fn render(canvas: &mut WindowCanvas, model: &model::Model) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::WHITE);
    let _ = canvas.fill_rect(Rect::new(
        model.x - (model.width / 2) as i32,
        model.y - (model.height / 2) as i32,
        model.width,
        model.height,
    ));
    canvas.present();
}
