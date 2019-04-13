use sdl2::keyboard::Scancode;
use std::time::Duration;

use super::fysiikka::Fysiikallinen;
use super::fysiikka::Fysiikkakappale;
use super::maailma::*;
use super::syotteet::*;

/// Huolehtii pelin toiminnasta esim. pelimaailman alustuksesta ja pelin päivityksestä
pub trait Paivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, events: &sdl2::EventPump);

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, paivitys_aika: &Duration);
}

/// Simppeli päivitys, joka huolehtii pelin toiminnasta
pub struct Peruspaivitys {
    pelihahmon_paivitys: PelihahmonPaivitys,
}

struct PelihahmonPaivitys;
const OIKEALLE_LIIKKUMINEN: Scancode = Scancode::Right;
const VASEMMALLE_LIIKKUMINEN: Scancode = Scancode::Left;
const ALAS_LIIKKUMINEN: Scancode = Scancode::Down;
const YLOS_LIIKKUMINEN: Scancode = Scancode::Up;
const AMPUMINEN: Scancode = Scancode::Space;

impl Paivitys for PelihahmonPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(&self, _maailma: &mut Maailma, syotteet: &mut Syotteet, events: &sdl2::EventPump) {
        syotteet.lisaa_nappain(events, OIKEALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, VASEMMALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, YLOS_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, ALAS_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, AMPUMINEN);
    }

    /// Päivittää pelihahmon tilan
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, paivitysaika: &Duration) {
        if maailma.onko_pelihahmo() {
            let mut x = 0.0;
            let mut y = 0.0;

            let liike = paivitysaika.as_micros() as f32 * 0.0002;
            if syotteet.nappain_pohjassa(OIKEALLE_LIIKKUMINEN) {
                x += liike;
            }
            if syotteet.nappain_pohjassa(VASEMMALLE_LIIKKUMINEN) {
                x -= liike;
            }
            if syotteet.nappain_pohjassa(YLOS_LIIKKUMINEN) {
                y -= liike;
            }
            if syotteet.nappain_pohjassa(ALAS_LIIKKUMINEN) {
                y += liike;
            }
            maailma
                .anna_pelihahmo_mut()
                .unwrap()
                .borrow_mut()
                .sijainti
                .liiku(x, y);

            let hahmon_sijainti = maailma.anna_pelihahmo().unwrap().borrow().sijainti;

            if syotteet.nappain_painettu(AMPUMINEN) {
                let kappale = maailma.lisaa_kappale(Kappale::new(
                    Muoto::Nelio(5.0, 5.0),
                    hahmon_sijainti.x + 22.5,
                    hahmon_sijainti.y + 10.0,
                ));
                maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(Vektori::new(80.0, 0.0), kappale));
            }
        }
    }
}

impl Default for Peruspaivitys {
    fn default() -> Self {
        Self::new()
    }
}

impl Peruspaivitys {
    /// Luo uuden peruspäivityksen
    pub fn new() -> Self {
        Peruspaivitys {
            pelihahmon_paivitys: PelihahmonPaivitys,
        }
    }
}

impl Paivitys for Peruspaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, events: &sdl2::EventPump) {
        maailma.lisaa_pelihahmo(Kappale::new(Muoto::Nelio(20.0, 20.0), 320.0, 240.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 470.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 10.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 10.0, 240.0));
        let rk = maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 630.0, 240.0));
        maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(Vektori::new(30.0, 0.0), rk));

        self.pelihahmon_paivitys.alusta(maailma, syotteet, events);
    }
    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, paivitysaika: &Duration) {
        self.pelihahmon_paivitys
            .paivita(maailma, syotteet, paivitysaika);
        for f in maailma.fysiikalliset() {
            f.paivita_sijainti(paivitysaika);
        }
    }
}
