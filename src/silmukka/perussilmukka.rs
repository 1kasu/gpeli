extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use super::super::maailma::*;
use super::super::paivitys::*;
use super::super::piirtaja::*;
use super::super::syotteet::*;
use super::Paasilmukka;
//use std::fs::File;
//use std::io::LineWriter;
//use std::io::Write;

/// Perussilmukka, joka päivittää peliä ja piirtää sen niin nopeasti kuin pystytään hyödyntäen päivitysaikaa
pub struct Perussilmukka<'a> {
    /// Tältä voidaan kysellä tapahtumia kuten näppäimen painalluksia
    events: sdl2::EventPump,
    /// Sdl context, jota tarvitaan esim. ajastimien luomisessa
    context: sdl2::Sdl,
    /// Osa, joka vastaa pelitilan esittämisestä käyttäjälle
    piirtaja: &'a mut Piirtaja,
    /// Pelin käyttämät syötteet
    syotteet: Syotteet,
    /// Pelin käyttämä päivitys
    paivitys: &'a Paivitys,
}

impl<'a> Perussilmukka<'a> {
    /// Luo uuden perussilmukan
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    /// * `paivitys` - Pelin käyttämä päivitys
    pub fn new(
        events: sdl2::EventPump,
        context: sdl2::Sdl,
        piirtaja: &'a mut Piirtaja,
        paivitys: &'a Paivitys,
    ) -> Self {
        Perussilmukka {
            events: events,
            context: context,
            piirtaja: piirtaja,
            syotteet: Syotteet::new(),
            paivitys: paivitys,
        }
    }
}

impl<'a> Paasilmukka for Perussilmukka<'a> {
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

        let mut maailma = Perusmaailma::new();
        self.paivitys
            .alusta(&mut maailma, &mut self.syotteet, &self.events);

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
            vanha_peliaika = peliaika;

            self.syotteet.paivita_nappainten_tilat(&self.events);

            /*if paivitysaika.as_nanos() > 5_000_000 {
                println!("{:?}", paivitysaika.as_nanos());
            }*/
            /*match write!(file, "{:?}\n", paivitysaika.as_nanos()) {
                Ok(_) => (),
                Err(_) => return Err("Tiedostoon ei voitu kirjoittaa".to_string()),
            }*/

            //if paivitysaika.as_micros() > 20000 {println!("{:?}", paivitysaika.as_micros());}

            self.paivitys
                .paivita(&mut maailma, &mut self.syotteet, &paivitysaika);

            maailma.poista_poistettavat();
            
            self.piirtaja.piirra_maailma(&maailma)?;
        }

        Ok(())
    }
}
