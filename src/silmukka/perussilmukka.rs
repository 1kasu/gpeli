extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use super::Paasilmukka;
use crate::maailma::*;
use crate::paivitys::*;
use crate::piirtaja::*;
use crate::syotteet::*;

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
    paivitys: &'a mut Paivitys,
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
        paivitys: &'a mut Paivitys,
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

        let mut maailma = Perusmaailma::new();
        self.paivitys
            .alusta(&mut maailma, &mut self.syotteet, &self.events);

        'paasilmukka: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'paasilmukka;
                    }
                    _ => {}
                }
            }
            //timer.delay(10);
            // Lasketaan paivitysaika
            peliaika = Instant::now();
            paivitysaika = peliaika.duration_since(vanha_peliaika);
            vanha_peliaika = peliaika;

            self.syotteet.paivita_nappainten_tilat(&self.events);

            self.paivitys
                .paivita(&mut maailma, &mut self.syotteet, &paivitysaika);

            maailma.poista_poistettavat();

            self.piirtaja.piirra_maailma(&maailma)?;
        }

        Ok(())
    }
}

impl<'a> std::fmt::Display for Perussilmukka<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Perussilmukka, joka päivittää viime päivityksestä kuluneen ajan mukaan")
    }
}
