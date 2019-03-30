extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::keyboard::Scancode;

pub trait Paasilmukka {
    fn new(
        canvas: Canvas<sdl2::video::Window>,
        events: sdl2::EventPump,
        context: sdl2::Sdl,
    ) -> Self;

    fn kaynnista_silmukka(&mut self) -> Result<(), String>;
}

pub struct Perussilmukka {
    canvas: Canvas<sdl2::video::Window>,
    events: sdl2::EventPump,
    context: sdl2::Sdl,
}

impl Paasilmukka for Perussilmukka {
    fn new(
        canvas: Canvas<sdl2::video::Window>,
        events: sdl2::EventPump,
        context: sdl2::Sdl,
    ) -> Self {
        Perussilmukka {
            canvas: canvas,
            events: events,
            context: context,
        }
    }

    fn kaynnista_silmukka(&mut self) -> Result<(), String> {
        let mut timer = self.context.timer()?;
        let mut peliaika = timer.ticks();
        let mut vanha_peliaika = peliaika;
        let mut paivitysaika: f32;
        
        let mut nelio = Rect::new(20, 20, 20, 20);
        let mut x = 20.0;
        let mut y = -20.0;
        
        'paa: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'paa;
                    }
                    _ => {}
                }
            }
            // Lasketaan paivitysaika
            peliaika = timer.ticks();
            paivitysaika = (peliaika - vanha_peliaika) as f32;
            vanha_peliaika = peliaika;

            self.canvas.set_draw_color(Color::RGB(10, 100, 10));
            self.canvas.clear();

            let liike = paivitysaika * 0.2;
            if self.events.keyboard_state().is_scancode_pressed(Scancode::Right){
                x += liike;
            }
            if self.events.keyboard_state().is_scancode_pressed(Scancode::Left){
                x -= liike;
            }
            if self.events.keyboard_state().is_scancode_pressed(Scancode::Up){
                y += liike;
            }
            if self.events.keyboard_state().is_scancode_pressed(Scancode::Down){
                y -= liike;
            }
            nelio.set_y(-y as i32);
            nelio.set_x(x as i32);
            self.canvas.set_draw_color(Color::RGB(200, 100, 10));
            self.canvas.fill_rect(Some(nelio))?;
            self.canvas.present();
        }

        Ok(())
    }
}
