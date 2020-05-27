extern crate sdl2;

use self::sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::model;
use crate::shape;

pub fn render(canvas: &mut WindowCanvas, model: &model::Model) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    render_bat(canvas, &model.bat);
    canvas.present();
}

fn render_bat(canvas: &mut WindowCanvas, bat: &shape::Shape) {
    canvas.set_draw_color(Color::WHITE);
    let _ = canvas.fill_rect(Rect::new(
        bat.x - (bat.width / 2) as i32,
        bat.y - (bat.height / 2) as i32,
        bat.width,
        bat.height,
    ));
}
