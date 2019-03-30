extern crate sdl2;

use sdl2::pixels::Color;

mod silmukka;
use crate::silmukka::Paasilmukka;
use crate::silmukka::Perussilmukka;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Peli", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

    let events = sdl_context.event_pump()?;

    let mut silmukka: Perussilmukka = Paasilmukka::new(canvas, events, sdl_context);

    silmukka.kaynnista_silmukka()
}
