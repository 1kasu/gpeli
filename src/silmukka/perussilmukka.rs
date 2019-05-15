extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use super::Paasilmukka;
use crate::maailma::*;
use crate::paivitys::*;
use crate::piirtaja::*;
use crate::syotteet::*;

/// Perussilmukka, joka päivittää peliä ja piirtää sen niin nopeasti kuin pystytään hyödyntäen päivitysaikaa
pub struct Perussilmukka<'a, T: MaailmanPiirtaja + ValiaikaistenPiirtaja> {
    /// Tältä voidaan kysellä tapahtumia kuten näppäimen painalluksia
    events: sdl2::EventPump,
    /// Sdl context, jota tarvitaan esim. ajastimien luomisessa
    context: sdl2::Sdl,
    /// Osa, joka vastaa pelitilan esittämisestä käyttäjälle
    piirtaja: &'a mut (T),
    /// Pelin käyttämät syötteet
    syotteet: Syotteet,
    /// Pelin käyttämä päivitys
    paivitys: &'a mut Paivitys,
}

impl<'a, T: MaailmanPiirtaja + ValiaikaistenPiirtaja> Perussilmukka<'a, T> {
    /// Luo uuden perussilmukan
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    /// * `paivitys` - Pelin käyttämä päivitys
    pub fn new(
        events: sdl2::EventPump,
        context: sdl2::Sdl,
        piirtaja: &'a mut T,
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

impl<'a, T: MaailmanPiirtaja + ValiaikaistenPiirtaja> Paasilmukka for Perussilmukka<'a, T> {
    /// Käynnistää pääsilmukan ja pyörittää sitä niin kauan kuin se vain pyörii
    fn kaynnista_silmukka(&mut self) -> Result<(), String> {
        // Alustetaan aikaan liittyvät muuttujat
        let mut _timer = self.context.timer()?;
        let mut peliaika = Instant::now();
        let mut kokonaisaika_pelin_alusta = Duration::new(0, 0);
        let mut vanha_peliaika = peliaika;
        let mut paivitysaika;

        // Alustetaan maailma
        let mut maailma = Perusmaailma::new();
        self.paivitys
            .alusta(&mut maailma, &mut self.syotteet, &self.events);

        // Varsinainen pääsilmukka
        'paasilmukka: loop {
            // Kerätään tapahtumat
            for event in self.events.poll_iter() {
                match event {
                    // Tarkistetaan suljetaanko peli (esim. ikkunan X klikkaamalla tai Esc näppäintä painamalla)
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        // Poistutaan pääsilmukasta eli käytännössä lopetetaan peli
                        break 'paasilmukka;
                    }
                    _ => {}
                }
            }
            // Lasketaan paivitysaika ja päivitetään kokonaisaikaa pelin alusta
            peliaika = Instant::now();
            paivitysaika = peliaika.duration_since(vanha_peliaika);
            kokonaisaika_pelin_alusta += paivitysaika;
            vanha_peliaika = peliaika;

            // Päivitetään syötteiden tilaa
            self.syotteet.paivita_nappainten_tilat(&self.events);

            // Päivitetään maailman tilaa
            self.paivitys.paivita(
                &mut maailma,
                &mut self.syotteet,
                &Paivitysaika::new(&paivitysaika, &kokonaisaika_pelin_alusta),
            );

            // Poistetaan maailmasta poistettaviksi merkityt kappaleet
            maailma.poista_poistettavat();

            // Piirretään maailma ja animaatiot
            self.piirtaja.puhdista_kuva();
            self.piirtaja.piirra_maailma(&maailma)?;
            self.piirtaja.piirra_kappaleista(&maailma.animaatio_kuva)?;
            self.piirtaja.esita_kuva();
        }

        Ok(())
    }
}

impl<'a, T: MaailmanPiirtaja + ValiaikaistenPiirtaja> std::fmt::Display for Perussilmukka<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Perussilmukka, joka päivittää viime päivityksestä kuluneen ajan mukaan"
        )
    }
}
