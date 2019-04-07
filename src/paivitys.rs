use sdl2::keyboard::Scancode;
use std::time::Duration;

use super::maailma::*;
use super::syotteet::Syotteet;

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
pub struct Peruspaivitys;

const OIKEALLE_LIIKKUMINEN: Scancode = Scancode::Right;
const VASEMMALLE_LIIKKUMINEN: Scancode = Scancode::Left;
const ALAS_LIIKKUMINEN: Scancode = Scancode::Down;
const YLOS_LIIKKUMINEN: Scancode = Scancode::Up;

impl Paivitys for Peruspaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, events: &sdl2::EventPump) {
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 20.0), 320.0, 240.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 470.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 10.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 10.0, 240.0));
        maailma.lisaa_kappale(Kappale::new(Muoto::Nelio(20.0, 480.0), 630.0, 240.0));

        syotteet.lisaa_nappain(events, OIKEALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, VASEMMALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, YLOS_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, ALAS_LIIKKUMINEN);
    }
    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(&self, maailma: &mut Maailma, syotteet: &mut Syotteet, paivitysaika: &Duration) {
        let mut x = 0.0;
        let mut y = 0.0;

        let liike = paivitysaika.as_micros() as f32 * 0.0002;
        if syotteet
            .anna_nappaimen_tila(OIKEALLE_LIIKKUMINEN)
            .map_or(false, |x| x.pohjassa())
        {
            x += liike;
        }
        if syotteet
            .anna_nappaimen_tila(VASEMMALLE_LIIKKUMINEN)
            .map_or(false, |x| x.pohjassa())
        {
            x -= liike;
        }
        if syotteet
            .anna_nappaimen_tila(YLOS_LIIKKUMINEN)
            .map_or(false, |x| x.pohjassa())
        {
            y -= liike;
        }
        if syotteet
            .anna_nappaimen_tila(ALAS_LIIKKUMINEN)
            .map_or(false, |x| x.pohjassa())
        {
            y += liike;
        }
        maailma.kappaleet[0].sijainti.liiku(x, y);
    }
}
