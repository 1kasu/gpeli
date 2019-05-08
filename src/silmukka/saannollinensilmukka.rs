extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use super::Paasilmukka;
use crate::maailma::*;
use crate::paivitys::*;
use crate::piirtaja::*;
use crate::syotteet::*;

/// Pääsilmukka, joka päivittää pelin tilaa säännöllisin väliajoin
pub struct SaannollinenSilmukka<'a> {
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
    /// Kuinka usein päivitys tehdään
    paivitysvali: Duration,
}

impl<'a> SaannollinenSilmukka<'a> {
    /// Luo uuden säännöllisesti päivittyvän silmukan
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    /// * `paivitys` - Pelin käyttämä päivitys
    /// * `paivitys_tiheys` - Kuinka monta kertaa sekunnissa päivitys tehdään
    pub fn new(
        events: sdl2::EventPump,
        context: sdl2::Sdl,
        piirtaja: &'a mut Piirtaja,
        paivitys: &'a mut Paivitys,
        paivitys_tiheys: u32,
    ) -> Self {
        SaannollinenSilmukka {
            events: events,
            context: context,
            piirtaja: piirtaja,
            syotteet: Syotteet::new(),
            paivitys: paivitys,
            paivitysvali: Duration::new(0, 1_000_000_000 / paivitys_tiheys),
        }
    }
}

impl<'a> Paasilmukka for SaannollinenSilmukka<'a> {
    /// Käynnistää pääsilmukan ja pyörittää sitä niin kauan kuin se vain pyörii
    fn kaynnista_silmukka(&mut self) -> Result<(), String> {
        let mut _timer = self.context.timer()?;
        let mut peliaika = Instant::now();
        let mut kokonaisaika_pelin_alusta = Duration::new(0, 0);
        let mut vanha_peliaika = peliaika;
        let mut paivitysaika;

        let lepoaika = 5;

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
            // Lasketaan paivitysaika
            peliaika = Instant::now();
            paivitysaika = peliaika.duration_since(vanha_peliaika);

            if paivitysaika < self.paivitysvali {
                if (self.paivitysvali - paivitysaika).as_micros() > lepoaika {
                    _timer.delay(lepoaika as u32);
                }
                continue;
            } else {
                paivitysaika = self.paivitysvali
            }

            vanha_peliaika = peliaika;
            kokonaisaika_pelin_alusta += paivitysaika;

            self.syotteet.paivita_nappainten_tilat(&self.events);

            self.paivitys
                .paivita(&mut maailma, &mut self.syotteet, &Paivitysaika::new(&paivitysaika, &kokonaisaika_pelin_alusta));

            maailma.poista_poistettavat();

            self.piirtaja.piirra_maailma(&maailma)?;
        }

        Ok(())
    }
}

impl<'a> std::fmt::Display for SaannollinenSilmukka<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Säännöllisesti päivittyvä silmukka, jonka päivitysväli on {} microsekuntia.",
            self.paivitysvali.as_micros()
        )
    }
}
