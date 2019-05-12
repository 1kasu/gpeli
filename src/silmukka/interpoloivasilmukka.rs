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
/// Säännöllinen päivitys tehdään niin monta kertaa kuin niitä mahtuu päivitysaikaan.
/// Esim. jos aikaa on kulunut viimeisestä säännöllisestä päivityksestä 3,6 kertaa päivitysväli,
/// niin suoritetaan 3 säännöllistä päivitystä.
pub struct InterpoloivaSilmukka<'a> {
    /// Tältä voidaan kysellä tapahtumia kuten näppäimen painalluksia
    events: sdl2::EventPump,
    /// Sdl context, jota tarvitaan esim. ajastimien luomisessa
    context: sdl2::Sdl,
    /// Osa, joka vastaa pelitilan esittämisestä käyttäjälle
    piirtaja: &'a mut ValiaikaistenPiirtaja,
    /// Pelin käyttämät syötteet
    syotteet: Syotteet,
    /// Pelin käyttämä säännöllinen päivitys
    saannollinen_paivitys: &'a mut Paivitys,
    /// Pelin käyttämä epäsäännöllinen päivitys
    epasaannollinen_paivitys: &'a mut Paivitys,
    /// Kuinka usein päivitys tehdään
    paivitysvali: Duration,
    /// Käytännössä arvo 0 tai 1. 0, jos interpoloidaan ja 1, jos ekstrapoloidaan.
    ekstrapolointi_lisa: f32,
}

impl<'a> InterpoloivaSilmukka<'a> {
    /// Luo uuden silmukan, jolla on sekä säännöllinen, että epäsäännöllinen päivitys
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    /// * `paivitys` - Pelin käyttämä päivitys
    /// * `paivitys_tiheys` - Kuinka monta kertaa sekunnissa päivitys tehdään
    pub fn new(
        events: sdl2::EventPump,
        context: sdl2::Sdl,
        piirtaja: &'a mut ValiaikaistenPiirtaja,
        saannollinen_paivitys: &'a mut Paivitys,
        epasaannollinen_paivitys: &'a mut Paivitys,
        paivitys_tiheys: u32,
    ) -> Self {
        InterpoloivaSilmukka {
            events: events,
            context: context,
            piirtaja: piirtaja,
            syotteet: Syotteet::new(),
            saannollinen_paivitys: saannollinen_paivitys,
            epasaannollinen_paivitys: epasaannollinen_paivitys,
            paivitysvali: Duration::new(0, 1_000_000_000 / paivitys_tiheys),
            ekstrapolointi_lisa: 0.0,
        }
    }

    /// Luo uuden silmukan, jolla on sekä säännöllinen, että epäsäännöllinen päivitys
    /// # Arguments
    /// * `events` - Eventpump, jolta saadaan tapahtumat
    /// * `context` - SDL2 konteksti
    /// * `piirtaja` - Osa, joka huolehtii pelin piirtämisestä
    /// * `paivitys` - Pelin käyttämä päivitys
    /// * `paivitys_tiheys` - Kuinka monta kertaa sekunnissa päivitys tehdään
    pub fn new_ekstrapoloiva(
        events: sdl2::EventPump,
        context: sdl2::Sdl,
        piirtaja: &'a mut ValiaikaistenPiirtaja,
        saannollinen_paivitys: &'a mut Paivitys,
        epasaannollinen_paivitys: &'a mut Paivitys,
        paivitys_tiheys: u32,
    ) -> Self {
        InterpoloivaSilmukka {
            events: events,
            context: context,
            piirtaja: piirtaja,
            syotteet: Syotteet::new(),
            saannollinen_paivitys: saannollinen_paivitys,
            epasaannollinen_paivitys: epasaannollinen_paivitys,
            paivitysvali: Duration::new(0, 1_000_000_000 / paivitys_tiheys),
            ekstrapolointi_lisa: 1.0,
        }
    }
}

impl<'a> Paasilmukka for InterpoloivaSilmukka<'a> {
    /// Käynnistää pääsilmukan ja pyörittää sitä niin kauan kuin se vain pyörii
    fn kaynnista_silmukka(&mut self) -> Result<(), String> {
        let mut _timer = self.context.timer()?;
        let mut peliaika = Instant::now();
        let mut kokonaisaika_pelin_alusta = Duration::new(0, 0);
        let mut kokonaisaika_pelin_alusta_saannollinen = Duration::new(0, 0);
        let mut vanha_peliaika = peliaika;
        // Kuinka kauan aikaa ennen kuin seuraava säännöllinen päivitys tehdään
        let mut aikaa_seuraavaan_saannolliseen_paivitykseen = self.paivitysvali;
        let mut paivitysaika;

        let mut maailma = Perusmaailma::new_interpoloiva();
        self.saannollinen_paivitys
            .alusta(&mut maailma, &mut self.syotteet, &self.events);
        self.epasaannollinen_paivitys
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
            kokonaisaika_pelin_alusta += paivitysaika;
            aikaa_seuraavaan_saannolliseen_paivitykseen += paivitysaika;
            vanha_peliaika = peliaika;

            // Päivitetään syötteet
            self.syotteet.paivita_nappainten_tilat(&self.events);

            // Toteutetaan niin, monta säännöllistä päivitystä, kuin mitä ollaan jääty jälkeen
            while aikaa_seuraavaan_saannolliseen_paivitykseen >= self.paivitysvali {
                kokonaisaika_pelin_alusta_saannollinen += self.paivitysvali;
                // Suoritetaan säännöllinen päivitys
                self.saannollinen_paivitys.paivita(
                    &mut maailma,
                    &mut self.syotteet,
                    &Paivitysaika::new(&self.paivitysvali, &kokonaisaika_pelin_alusta_saannollinen),
                );

                maailma.paivita_kappalemuistia();

                aikaa_seuraavaan_saannolliseen_paivitykseen -= self.paivitysvali;
            }

            // Tehdään epäsäännöllinen päivitys
            self.epasaannollinen_paivitys.paivita(
                &mut maailma,
                &mut self.syotteet,
                &Paivitysaika::new(&paivitysaika, &kokonaisaika_pelin_alusta),
            );

            maailma.poista_poistettavat();

            let mut piirrettavat_kappaleet = Vec::new();

            maailma.aseta_interpolaatio_arvo(
                self.ekstrapolointi_lisa
                    + aikaa_seuraavaan_saannolliseen_paivitykseen.as_micros() as f32
                        / self.paivitysvali.as_micros() as f32,
            );

            if let Some(kamera) = maailma.anna_kameran_sijainti() {
                self.piirtaja.aseta_kameran_sijainti(kamera)?;
            }

            maailma.anna_piirrettavat(&mut piirrettavat_kappaleet);

            // Piirretään maailma ja animaatiot
            self.piirtaja.puhdista_kuva();
            self.piirtaja.piirra_kappaleista(&piirrettavat_kappaleet)?;
            self.piirtaja.piirra_kappaleista(&maailma.animaatio_kuva)?;
            self.piirtaja.esita_kuva();
        }

        Ok(())
    }
}

impl<'a> std::fmt::Display for InterpoloivaSilmukka<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Interpoloiva silmukka, joka päivittää säännöllisen päivityksen välillä {} microsekuntia. Epäsäännöllinen päivitys taas niin usein kuin mahdollista.",
            self.paivitysvali.as_micros()
        )
    }
}
