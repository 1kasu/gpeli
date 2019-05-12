use sdl2::pixels::Color;
use std::rc::Rc;
use std::time::Duration;

use crate::animointi::{AmmusAnimaatio, KatoamisAnimaatio, Kuolevainen};
use crate::fysiikka::{Fysiikallinen, Fysiikka, Fysiikkakappale, Tormaystiedot, Tormaystieto};
use crate::maailma::kappale::{Kappale, Muoto, Tagi::*};
use crate::maailma::pelihahmo::Pelihahmo;
use crate::maailma::vektori::Vektori;
use crate::maailma::*;
use crate::paivitys::{Paivitys, Paivitysaika};
use crate::piirtaja::Piirtotapa;
use crate::spawneri::Spawneri;
use crate::syotteet::*;
use crate::tekoaly::{Alyllinen, SeurausAly};
use crate::tormays::{Tormaystoiminta, YleinenTormays};

use super::{lisaa_fysiikka_kappale, lisaa_kappale};

/// Simppeli päivitys, joka huolehtii pelin toiminnasta
pub struct SpawnerinPaivitys {
    spawnerit: Vec<Spawneri>,
}

impl Default for SpawnerinPaivitys {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnerinPaivitys {
    /// Luo uuden peruspäivityksen
    pub fn new() -> Self {
        SpawnerinPaivitys {
            spawnerit: Default::default(),
        }
    }
}

impl Paivitys for SpawnerinPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        _maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        _events: &sdl2::EventPump,
    ) {
        self.spawnerit.push(Spawneri::new(
            Duration::new(5, 0),
            Kappale::new_keskipisteella(Muoto::Nelio(20.0, 20.0), 600.0, 540.0, Vihollinen),
            Piirtotapa::Yksivarinen {
                vari: Color::RGB(0, 0, 0),
            },
            Some(Default::default()),
            Some(Box::new(SeurausAly)),
        ));
    }

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `_syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        paivitysaika: &Paivitysaika,
    ) {
        for spawneri in &mut self.spawnerit {
            spawneri.paivita_spawneria(maailma, paivitysaika);
        }
    }
}

/// Päivittää maailmassa olevia tekoälyjä
pub struct AnimaatioidenPaivitys;

impl Paivitys for AnimaatioidenPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        _maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        _events: &sdl2::EventPump,
    ) {

    }

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `_syotteet` - Päivityksessä käytettävät syötteet
    /// * `_paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        paivitysaika: &Paivitysaika,
    ) {
        maailma.animaatio_kuva = Default::default();
        maailma
            .animaatiot
            .anna_piirrettavat(&mut maailma.animaatio_kuva, paivitysaika);
    }
}

/// Päivittää maailmassa olevia tekoälyjä
pub struct TekoalynPaivitys;

impl Paivitys for TekoalynPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        _maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        _events: &sdl2::EventPump,
    ) {

    }

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `_syotteet` - Päivityksessä käytettävät syötteet
    /// * `_paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        _paivitysaika: &Paivitysaika,
    ) {
        maailma.laske_tekoalyt();
    }
}

/// Pelin fysiikan päivitys ja törmäyskäsittely
pub struct FysiikanPaivitys;

impl Paivitys for FysiikanPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `_maailma` - Pelimaailma, joka alustetaan
    /// * `_syotteet` - Alustettavat syotteet
    /// * `_events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        _events: &sdl2::EventPump,
    ) {
        // Pelihahmo
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new_keskipisteella(Muoto::Nelio(20.0, 20.0), 320.0, 240.0, Pelaaja),
            Color::RGB(255, 30, 30),
        );
        maailma.lisaa_pelihahmo(Pelihahmo::new(Rc::clone(&_rk)));
        maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(Default::default(), _rk));

        lisaa_kappale(
            maailma,
            Kappale::new_keskipisteella(Muoto::Ympyra(30.0), 0.0, 0.0, Seina),
            Color::RGB(200, 200, 200),
        );

        // Seinät
        let origo: Vektori = Default::default();
        let seinan_paksuus = 40.0;
        let x_pituus = 1000.0;
        let y_pituus = 700.0;
        let esteiden_vari = Color::RGB(10, 100, 200);
        // Luodaan seinät
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new_kulmalla(
                Muoto::Nelio(x_pituus, seinan_paksuus),
                origo.x,
                origo.y,
                Seina,
            ),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new_kulmalla(
                Muoto::Nelio(x_pituus, seinan_paksuus),
                origo.x,
                origo.y + seinan_paksuus + y_pituus,
                Seina,
            ),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new_kulmalla(
                Muoto::Nelio(seinan_paksuus, y_pituus),
                origo.x,
                origo.y + seinan_paksuus,
                Seina,
            ),
            esteiden_vari,
        );
        let _rk = lisaa_fysiikka_kappale(
            maailma,
            Kappale::new_kulmalla(
                Muoto::Nelio(seinan_paksuus, y_pituus),
                origo.x + x_pituus - seinan_paksuus,
                origo.y + seinan_paksuus,
                Seina,
            ),
            esteiden_vari,
        );

        // Luodaan AI hahmo
        let _rk = lisaa_kappale(
            maailma,
            Kappale::new_keskipisteella(Muoto::Nelio(20.0, 20.0), 600.0, 540.0, Vihollinen),
            Color::RGB(0, 0, 0),
        );
        maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(Default::default(), Rc::clone(&_rk)));
        maailma.lisaa_aly(Alyllinen::new(_rk, Box::new(SeurausAly)));
    }

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        _syotteet: &mut Syotteet,
        paivitysaika: &Paivitysaika,
    ) {
        let mut fysiikka = Fysiikka::new();
        fysiikka.laske_uudet_sijainnit(maailma.fysiikalliset(), paivitysaika.paivitysaika);

        TormaystenKasittely::kasittele_tormaykset(fysiikka.tormaykset, maailma, &paivitysaika);
    }
}

pub struct TormaystenKasittely;

impl TormaystenKasittely {
    fn kasittele_tormaykset(
        tormaykset: Tormaystiedot,
        maailma: &mut Perusmaailma,
        paivitysaika: &Paivitysaika,
    ) {
        let mut mahdolliset_tapahtumat = Vec::new();
        mahdolliset_tapahtumat.push(YleinenTormays::new(
            vec![Ammus],
            vec![Seina, Vihollinen, Ammus, Pelaaja],
            &tuhoa_tormaaja,
        ));
        mahdolliset_tapahtumat.push(YleinenTormays::new(
            vec![Vihollinen],
            vec![Ammus],
            &animaatio_tuhoutuminen,
        ));
        for tormays in tormaykset.anna_tormaykset() {
            for toiminta in &mahdolliset_tapahtumat {
                if toiminta.ehto(maailma.fysiikalliset()[tormays.indeksi].anna_tagi()) {
                    toiminta.toiminta(tormays, maailma, paivitysaika);
                }
            }
        }
    }
}

/// Tuhoaa törmääjän
/// # Arguments
/// * `tormays` - Törmäystapahtuman tiedot
/// * `maailma` - Maailma, jossa törmäysta pahtui
/// * `paivitysaika` - Paivitysaika
fn tuhoa_tormaaja(tormays: &Tormaystieto, maailma: &mut Perusmaailma, paivitysaika: &Paivitysaika) {
    let f_kappale = &maailma.fysiikalliset()[tormays.indeksi];
    let suunta = f_kappale.anna_nopeus().yksikkovektori();
    //println!("Yritetään poistaa ammus");
    let kopio = f_kappale.anna_kappale();

    let animaation_kesto = Duration::new(0, 100_000_000);
    maailma.animaatiot.lisaa_animaatio(Kuolevainen::new(
        Box::new(AmmusAnimaatio::new(
            kopio.borrow().keskipisteen_sijainti(),
            *paivitysaika.kokonais_pelin_aika,
            suunta,
            animaation_kesto,
            Color::RGB(200, 0, 100),
        )),
        *paivitysaika.kokonais_pelin_aika + animaation_kesto,
    ));

    maailma.lisaa_poistettava(kopio);
}

fn animaatio_tuhoutuminen(
    tormays: &Tormaystieto,
    maailma: &mut Perusmaailma,
    paivitysaika: &Paivitysaika,
) {
    let f_kappale = &maailma.fysiikalliset()[tormays.indeksi];
    //println!("Yritetään poistaa ammus");
    let kopio = f_kappale.anna_kappale();

    let animaation_kesto = Duration::new(1, 0);
    maailma.animaatiot.lisaa_animaatio(Kuolevainen::new(
        Box::new(KatoamisAnimaatio::new(
            kopio.borrow().keskipisteen_sijainti(),
            *paivitysaika.kokonais_pelin_aika,
            kopio.borrow().muoto.koko().0,
            1.0,
            animaation_kesto,
            Color::RGB(200, 0, 100),
        )),
        *paivitysaika.kokonais_pelin_aika + animaation_kesto,
    ));

    maailma.lisaa_poistettava(kopio);
}
