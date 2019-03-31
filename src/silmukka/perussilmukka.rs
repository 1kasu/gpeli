extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

use super::Paasilmukka;
use super::maailma::*;


/// Perus silmukka, joka päivittää peliä ja piirtää sen niin nopeasti kuin pystytään hyödyntäen päivitysaikaa
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
        
        let mut maailma = Maailma::new();
        
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20,20), 20.0,20.0));

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

            let mut x = 0.0;
            let mut y = 0.0;

            let liike = paivitysaika * 0.2;
            if self
                .events
                .keyboard_state()
                .is_scancode_pressed(Scancode::Right)
            {
                x += liike;
            }
            if self
                .events
                .keyboard_state()
                .is_scancode_pressed(Scancode::Left)
            {
                x -= liike;
            }
            if self
                .events
                .keyboard_state()
                .is_scancode_pressed(Scancode::Up)
            {
                y -= liike;
            }
            if self
                .events
                .keyboard_state()
                .is_scancode_pressed(Scancode::Down)
            {
                y += liike;
            }
            maailma.kappaleet[0].sijainti.liiku(x,y);
            
            piirra_maailma(&mut self.canvas, &maailma)?;
        }

        Ok(())
    }
}

fn piirra_maailma(canvas: &mut Canvas<sdl2::video::Window>, maailma: &Maailma) -> Result<(), String> {
    
    canvas.set_draw_color(Color::RGB(10, 100, 10));
    canvas.clear();
    
    canvas.set_draw_color(Color::RGB(200, 100, 10));
    
    for kappale in &maailma.kappaleet{
        match kappale.muoto {
            Muoto::Nelio(leveys, korkeus) => {
                canvas.fill_rect(Some(Rect::new(kappale.sijainti.x as i32, kappale.sijainti.y as i32, leveys, korkeus)))?;
            },
            Muoto::Ympyra(_) => ()
        }    
    }
    canvas.present();
    
    Ok(())
}