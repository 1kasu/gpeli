extern crate sdl2;

use sdl2::pixels::Color;

pub mod maailma;
pub mod piirtaja;
pub mod silmukka;
use crate::piirtaja::*;
use crate::silmukka::perussilmukka::Perussilmukka;
use crate::silmukka::Paasilmukka;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Peli", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

    let events = sdl_context.event_pump()?;
    let mut piirtaja = Peruspiirtaja::new(canvas)?;
    
    let seuraus = (0.2,0.2);
    let zoomi = 1.0;
    println!("Seuraus on {0} ja {1}", seuraus.0, seuraus.1);
    println!("Zoomi on {}", zoomi);
    
    piirtaja.aseta_kameran_seurauksen_etaisyys(seuraus)?;
    piirtaja.aseta_kameran_zoomi(zoomi);

    let mut silmukka = Perussilmukka::new(events, sdl_context, piirtaja);
    silmukka.kaynnista_silmukka()
}
