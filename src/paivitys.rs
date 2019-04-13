use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::fysiikka::Fysiikallinen;
use crate::fysiikka::Fysiikkakappale;
use crate::maailma::*;
use crate::piirtaja::PiirrettavaKappale;
use crate::syotteet::*;

/// Selkeyttää koodia, kun arvataan, että vektorilla tarkoitetaan luotavan kappaleen nopeutta ja suuntaa.
type Nopeus = Vektori;

/// Huolehtii pelin toiminnasta esim. pelimaailman alustuksesta ja pelin päivityksestä
pub trait Paivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(&self, maailma: &mut Perusmaailma, syotteet: &mut Syotteet, events: &sdl2::EventPump);

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        paivitys_aika: &Duration,
    );
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
    fn alusta(
        &self,
        _maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        events: &sdl2::EventPump,
    ) {
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
    fn paivita(
        &self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        paivitysaika: &Duration,
    ) {
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
                .kappale
                .borrow_mut()
                .sijainti
                .liiku(x, y);

            let hahmon_sijainti = maailma.anna_pelihahmo().unwrap().kappale.borrow().sijainti;

            if syotteet.nappain_painettu(AMPUMINEN) {
                let r_kappale = lisaa_kappale(
                    maailma,
                    Kappale::new(
                        Muoto::Nelio(5.0, 5.0),
                        hahmon_sijainti.x + 22.5,
                        hahmon_sijainti.y + 10.0,
                    ),
                    Color::RGB(0, 255, 255),
                );
                maailma
                    .lisaa_fysiikkakappale(Fysiikkakappale::new(Nopeus::new(80.0, 0.0), r_kappale));
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

fn lisaa_kappale(
    maailma: &mut Perusmaailma,
    kappale: Kappale,
    vari: Color,
) -> Rc<RefCell<Kappale>> {
    let r_kappale = maailma.lisaa_kappale(kappale);
    maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::YksivarinenKappale {
        kappale: Rc::clone(&r_kappale),
        vari: vari,
    });
    r_kappale
}

impl Paivitys for Peruspaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        events: &sdl2::EventPump,
    ) {
        let esteiden_vari = Color::RGB(20, 20, 200);
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 20.0), 320.0, 240.0),
            Color::RGB(255, 30, 30),
        );
        maailma.lisaa_pelihahmo(Pelihahmo::new(Rc::clone(&_rk)));
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 470.0),
            esteiden_vari,
        );
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 10.0),
            esteiden_vari,
        );
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 480.0), 10.0, 240.0),
            esteiden_vari,
        );
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 480.0), 630.0, 240.0),
            esteiden_vari,
        );

        self.pelihahmon_paivitys.alusta(maailma, syotteet, events);
    }
    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        paivitysaika: &Duration,
    ) {
        self.pelihahmon_paivitys
            .paivita(maailma, syotteet, paivitysaika);
        for f in maailma.fysiikalliset() {
            f.paivita_sijainti(paivitysaika);
        }
    }
}
