use sdl2::pixels::Color;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use crate::maailma::kappale::Kappale;
use crate::maailma::kappale::Muoto::*;
use crate::maailma::kappale::Tagi::*;
use crate::maailma::vektori::Vektori;
use crate::piirtaja::{PiirrettavaKappale, PiirrettavaMaailma, Piirtotapa};

type Peliaika = Duration;

#[derive(Default)]
pub struct Animaatiot {
    animaatiot: Vec<Kuolevainen<Animaatio>>,
    piirrettavat_kappaleet: Vec<PiirrettavaKappale>,
}

impl Animaatiot {
    pub fn new() -> Self {
        Animaatiot {
            animaatiot: Default::default(),
            piirrettavat_kappaleet: Default::default(),
        }
    }

    pub fn lisaa_animaatio(&mut self, animaatio: Kuolevainen<Animaatio>) {
        self.animaatiot.push(animaatio);
    }

    pub fn paivita_animaatiot(&mut self, pelimaailman_aika: &Peliaika) {
        self.piirrettavat_kappaleet = Default::default();
        self.animaatiot.retain(|x| !x.kuoleeko(pelimaailman_aika));
        for a in &mut self.animaatiot {
            if let Some(aika) = pelimaailman_aika.checked_sub(*a.animaation_alku()) {
                a.anna_palat(&mut self.piirrettavat_kappaleet, &aika);
            }
        }
    }
}

impl PiirrettavaMaailma for Animaatiot {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat<'a>(
        &'a self,
        _sijainti: Vektori,
    ) -> Box<Iterator<Item = &'a PiirrettavaKappale> + 'a> {
        Box::new(self.piirrettavat_kappaleet.iter())
    }

    /// Antaa kameran sijainnin pelimaailmassa, jos maailma haluaa ehdottaa jotakin
    fn anna_kameran_sijainti(&self) -> Option<Vektori> {
        None
    }
}

pub struct Animaatio {
    animaation_alku: Peliaika,
    sijainti: Vektori,
}

impl Animaatio {
    pub fn new(sijainti: Vektori, animaation_alku: Peliaika) -> Self {
        Animaatio {
            animaation_alku: animaation_alku,
            sijainti: sijainti,
        }
    }

    pub fn sijainti_mut(&mut self) -> &mut Vektori {
        &mut self.sijainti
    }

    pub fn animaation_alku(&self) -> &Peliaika {
        &self.animaation_alku
    }

    pub fn anna_palat(&self, palat: &mut Vec<PiirrettavaKappale>, framen_aika: &Duration) {
        let frame_sekunteina = framen_aika.as_micros() as f32 / 1_000_000 as f32;
        let koko = anna_lineaarinen_interpolaatio(0.0, 20.0, 1.0, 1.0, frame_sekunteina);
        println!("{:?} {:?}", koko, frame_sekunteina);
        let a = PiirrettavaKappale::new(
            Rc::new(RefCell::new(Kappale::new_keskipisteella(
                Nelio(koko, koko),
                self.sijainti.x,
                self.sijainti.y,
                Partikkeli,
            ))),
            Piirtotapa::Yksivarinen {
                vari: Color::RGB(200, 0, 100),
            },
        );
        palat.push(a);
    }
}

pub struct Kuolevainen<T> {
    kuolin_aika: Peliaika,
    arvo: T,
}

impl<T> Kuolevainen<T> {
    pub fn new(sisalto: T, kuolin_aika: Peliaika) -> Self {
        Kuolevainen {
            arvo: sisalto,
            kuolin_aika: kuolin_aika,
        }
    }
    pub fn kuoleeko(&self, peliaika: &Peliaika) -> bool {
        self.kuolin_aika <= *peliaika
    }
}

impl<T> Deref for Kuolevainen<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.arvo
    }
}

pub fn anna_lineaarinen_interpolaatio(
    alku_x: f32,
    alku_y: f32,
    loppu_x: f32,
    loppu_y: f32,
    interpolaatio_arvo: f32,
) -> f32 {
    alku_y + (interpolaatio_arvo - alku_x) * (loppu_y - alku_y) / (loppu_x - alku_x)
}
