extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::time::Instant;

use super::super::maailma::*;
use super::super::piirtaja::*;
use super::Paasilmukka;
//use std::fs::File;
//use std::io::LineWriter;
//use std::io::Write;

/// Perussilmukka, joka päivittää peliä ja piirtää sen niin nopeasti kuin pystytään hyödyntäen päivitysaikaa
pub struct Perussilmukka {
    /// Tältä voidaan kysellä tapahtumia kuten näppäimen painalluksia
    events: sdl2::EventPump,
    /// Sdl context, jota tarvitaan esim. ajastimien luomisessa
    context: sdl2::Sdl,
    /// Osa, joka vastaa pelitilan esittämisestä käyttäjälle
    piirtaja: Peruspiirtaja,
}

impl Perussilmukka {
    /// Luo uuden perussilmukan
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    pub fn new(events: sdl2::EventPump, context: sdl2::Sdl, piirtaja: Peruspiirtaja) -> Self {
        Perussilmukka {
            events: events,
            context: context,
            piirtaja: piirtaja,
        }
    }
}

impl Paasilmukka for Perussilmukka {
    /// Käynnistää pääsilmukan ja pyörittää sitä niin kauan kuin se vain pyörii
    fn kaynnista_silmukka(&mut self) -> Result<(), String> {
        let mut _timer = self.context.timer()?;
        let mut peliaika = Instant::now();
        let mut vanha_peliaika = peliaika;
        let mut paivitysaika;

        /*let file = match File::create("paivitysaika.txt") {
            Ok(x) => x,
            Err(_) => return Err("Tiedostoa ei voitu luoda.".to_string()),
        };
        let mut _file = LineWriter::new(file);*/

        let mut maailma = Maailma::new();
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 20.0), 320.0, 240.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 470.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 10.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 10.0, 240.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 630.0, 240.0));

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
            //timer.delay(10);
            // Lasketaan paivitysaika
            peliaika = Instant::now();
            paivitysaika = vanha_peliaika.elapsed();
            if paivitysaika.as_nanos() > 5_000_000 {
                println!("{:?}", paivitysaika.as_nanos());
            }
            /*match write!(file, "{:?}\n", paivitysaika.as_nanos()) {
                Ok(_) => (),
                Err(_) => return Err("Tiedostoon ei voitu kirjoittaa".to_string()),
            }*/

            vanha_peliaika = peliaika;
            let mut x = 0.0;
            let mut y = 0.0;

            //if paivitysaika.as_micros() > 20000 {println!("{:?}", paivitysaika.as_micros());}
            let liike = paivitysaika.as_micros() as f32 * 0.0002;
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
            maailma.kappaleet[0].sijainti.liiku(x, y);

            self.piirtaja
                .aseta_kameran_sijainti(maailma.kappaleet[0].sijainti)?;
            self.piirtaja.piirra_maailma(&maailma)?;
        }

        Ok(())
    }
}
