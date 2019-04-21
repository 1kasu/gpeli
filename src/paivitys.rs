use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::fysiikka::Fysiikallinen;
use crate::fysiikka::Fysiikka;
use crate::fysiikka::Fysiikkakappale;
use crate::fysiikka::Tormaystiedot;
use crate::fysiikka::Tormaystieto;
use crate::maailma::Tagi::*;
use crate::maailma::*;
use crate::piirtaja::PiirrettavaKappale;
use crate::syotteet::*;

// Vakioita eri asioille
const OIKEALLE_LIIKKUMINEN: Scancode = Scancode::Right;
const VASEMMALLE_LIIKKUMINEN: Scancode = Scancode::Left;
const ALAS_LIIKKUMINEN: Scancode = Scancode::Down;
const YLOS_LIIKKUMINEN: Scancode = Scancode::Up;
const AMPUMINEN: Scancode = Scancode::Space;
const PELIHAHMON_NOPEUS: f32 = 120.0;
const AMMUKSEN_NOPEUS: f32 = 260.0;
const AMMUKSEN_LEVEYS: f32 = 30.0;
const AMMUKSEN_KORKEUS: f32 = 5.0;

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
        _paivitysaika: &Duration,
    ) {
        if let Some(pelihahmo) = maailma.anna_pelihahmo_mut() {
            let mut x = 0.0;
            let mut y = 0.0;

            // Liikutetaan pelihahmoa
            if syotteet.nappain_pohjassa(OIKEALLE_LIIKKUMINEN) {
                x += PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(VASEMMALLE_LIIKKUMINEN) {
                x -= PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(YLOS_LIIKKUMINEN) {
                y -= PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(ALAS_LIIKKUMINEN) {
                y += PELIHAHMON_NOPEUS;
            }

            let hahmon_kappale = pelihahmo.anna_kappale();
            let pelaajan_nopeus = Nopeus::new(x, y);

            if let Some(hahmon_fysiikka) = maailma.anna_fysiikka(&hahmon_kappale) {
                hahmon_fysiikka.aseta_nopeus(pelaajan_nopeus);
            }

            let pelihahmo = maailma.anna_pelihahmo_mut().unwrap();
            // Päivitetään suunta
            pelihahmo.aseta_suunta(pelaajan_nopeus);

            let hahmon_sijainti = hahmon_kappale.borrow().sijainti;

            // Pelihahmon ampuminen
            if syotteet.nappain_painettu(AMPUMINEN) {
                // Lasketaan lisättävän ammuksen sijainti
                let (pelaajan_leveys_puolikas, pelaajan_korkeus_puolikas) =
                    match hahmon_kappale.borrow().muoto {
                        Muoto::Nelio(leveys, korkeus) => (leveys / 2.0, korkeus / 2.0),
                        Muoto::Ympyra(sade) => (sade, sade),
                    };
                let pelaajan_keskipiste = hahmon_sijainti
                    + Vektori::new(pelaajan_leveys_puolikas, pelaajan_korkeus_puolikas);
                let ammuksen_keskipiste = Vektori::new(AMMUKSEN_LEVEYS, AMMUKSEN_KORKEUS / 2.0);
                let ammuksen_suunta = pelihahmo.anna_suunta();
                let muutos = ammuksen_suunta * 2.0 * pelaajan_leveys_puolikas - ammuksen_keskipiste;

                // Lisätään ammus pelaajan katsomissuuntaan vähän matkan päähän
                let r_kappale = lisaa_kappale(
                    maailma,
                    Kappale {
                        muoto: Muoto::Ympyra(AMMUKSEN_LEVEYS),
                        sijainti: pelaajan_keskipiste + muutos,
                        tagi: Ammus,
                    },
                    Color::RGB(0, 255, 255),
                );

                // Lisätään ammukselle fysiikka ja ammuksen alkunopeus
                maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(
                    ammuksen_suunta * AMMUKSEN_NOPEUS,
                    r_kappale,
                ));
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
        maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(Default::default(), _rk));

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

        TormaystenKasittely::kasittele_tormaykset(fysiikka.tormaykset, maailma)
    }
}

pub struct TormaystenKasittely;

impl TormaystenKasittely {
    fn kasittele_tormaykset(tormaykset: Tormaystiedot, maailma: &mut Perusmaailma) {
        let mut mahdolliset_tapahtumat = Vec::new();
        mahdolliset_tapahtumat.push(AmmustenTormays);
        for tormays in tormaykset.anna_tormaykset() {
            for toiminta in &mahdolliset_tapahtumat {
                if toiminta.ehto(maailma.fysiikalliset()[tormays.indeksi].anna_tagi()) {
                    toiminta.toiminta(tormays, maailma);
                }
            }
        }
    }
}

trait Tormaystoiminta {
    /// Tapahtuuko toiminto
    fn ehto(&self, tagi: Tagi) -> bool;
    /// Toiminta, joka tehdään ehdon toteutuessa
    fn toiminta(&self, tormays: &Tormaystieto, maailma: &mut Perusmaailma);
}

struct AmmustenTormays;

impl Tormaystoiminta for AmmustenTormays {
    fn ehto(&self, tagi: Tagi) -> bool {
        tagi == Ammus
    }

    fn toiminta(&self, tormays: &Tormaystieto, maailma: &mut Perusmaailma) {
        let f_kappale = &maailma.fysiikalliset()[tormays.indeksi];
        //println!("Yritetään poistaa ammus");
        let kopio = f_kappale.anna_kappale();

        if let Some(piirto) = maailma.anna_piirrettavyys(&kopio) {
            if let PiirrettavaKappale::YksivarinenKappale { ref mut vari, .. } = piirto {
                *vari = Color::RGB(239, 40, 117);
            }
        }

        //maailma.lisaa_poistettava(kopio);
    }
}
