extern crate sdl2;

use self::sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use self::sdl2::gfx::primitives::DrawRenderer;
use crate::model;
use crate::shape;

pub fn render(canvas: &mut WindowCanvas, model: &model::Model) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    render_bat(canvas, &model.shapes.get(model.index_bat).unwrap());
    render_ball(canvas, &model.shapes.get(model.index_ball).unwrap());
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

fn render_ball(canvas: &mut WindowCanvas, ball: &shape::Shape) {
    let _ = canvas.filled_circle(
        ball.x as i16,
        ball.y as i16,
        (ball.width / 2) as i16,
        Color::WHITE,
    );
}
