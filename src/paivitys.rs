use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::fysiikka::Fysiikka;
use crate::fysiikka::Fysiikkakappale;
use crate::maailma::Tagi::*;
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

            // Liikutetaan pelihahmoa
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

            // Pelihahmon ampuminen
            if syotteet.nappain_painettu(AMPUMINEN) {
                let r_kappale = lisaa_kappale(
                    maailma,
                    Kappale::new(
                        Muoto::Nelio(5.0, 5.0),
                        hahmon_sijainti.x + 22.5,
                        hahmon_sijainti.y + 10.0,
                        Ammus,
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

/// Lisää kappaleen maailmaan, luoden sille piirrettävän lisäosan
/// # Arguments
/// * `maailma` - Pelimaailma, johon kappale lisätään
/// * `kappale` - Lisättävä kappale
/// * `vari` - Lisättävän kappaleen väri
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

/// Lisää fysiikkakappaleen kappaleineen maailmaan
/// /// # Arguments
/// * `maailma` - Pelimaailma, johon kappale lisätään
/// * `kappale` - Lisättävä kappale
/// * `vari` - Lisättävän kappaleen väri
fn lisaa_fysiikka_kappale(
    maailma: &mut Perusmaailma,
    kappale: Kappale,
    vari: Color,
) -> Rc<RefCell<Kappale>> {
    let r_kappale = maailma.lisaa_kappale(kappale);
    maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::YksivarinenKappale {
        kappale: Rc::clone(&r_kappale),
        vari: vari,
    });
    let f_kappale = Fysiikkakappale::new(Default::default(), Rc::clone(&r_kappale));
    maailma.lisaa_fysiikkakappale(f_kappale);
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
        // Pelihahmo
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 20.0), 320.0, 240.0, Pelaaja),
            Color::RGB(255, 30, 30),
        );
        maailma.lisaa_pelihahmo(Pelihahmo::new(Rc::clone(&_rk)));

        // Seinät
        let esteiden_vari = Color::RGB(20, 20, 200);
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 470.0, Seina),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(640.0, 20.0), 320.0, 10.0, Seina),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 480.0), 10.0, 240.0, Seina),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new(Muoto::Nelio(20.0, 480.0), 630.0, 240.0, Seina),
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

        let mut fysiikka = Fysiikka::new();
        fysiikka.laske_uudet_sijainnit(maailma.fysiikalliset(), paivitysaika);

        //let mut poistettavat = Vec::new();
        for tormays in fysiikka.tormaykset.anna_tormaykset() {
            if maailma.fysiikalliset()[tormays.indeksi].anna_tagi() == Ammus {
                let f_kappale = &maailma.fysiikalliset()[tormays.indeksi];
                //println!("Yritetään poistaa ammus");
                let kopio = Rc::clone(f_kappale.anna_kappale());
                
                if let Some(piirto) = maailma.anna_piirrettavyys(&kopio){
                    if let PiirrettavaKappale::YksivarinenKappale{ref mut vari, ..} = piirto{
                        *vari = Color::RGB(198, 99, 137);
                    }
                    
                }
                
                //maailma.lisaa_poistettava(kopio);
            }
        }

        maailma.poista_poistettavat();
    }
}
