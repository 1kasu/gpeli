use std::cell::RefCell;
use std::rc::Rc;

use crate::fysiikka::Fysiikkakappale;
use crate::maailma::*;

type RcKappale = Rc<RefCell<Kappale>>;

/// Sisältää tekoälyn ja kappale, johon tekoäly viittaa
pub struct Alyllinen {
    /// Kappale, jolla tekoäly on toiminnassa
    kappale: RcKappale,
    /// Kappaleen käyttämä tekoäly
    aly: Box<Aly>,
}

/// Sisältää tekoälyn tarvitsemat tiedot maailmasta
pub struct TekoalyMaailma<'a> {
    /// Kappaleet
    _kappaleet: &'a mut Vec<RcKappale>,
    /// Pelaajan pelihahmo
    pelihahmo: &'a mut Option<Pelihahmo>,
    /// Fysiikan sisältämät kappaleet
    _fysiikalliset: &'a mut Vec<Fysiikkakappale>,
}

impl<'a> TekoalyMaailma<'a> {
    /// Luo uuden tekoälyn käyttämän maailman
    /// # Arguments
    /// * `_kappaleet` - Maailman kappaleet
    /// * `pelihahmo` - Pelaajan ohjaama hahmo
    /// * `_fysikaalliset` - Maailmassa olevat fysiikkakappaleet
    pub fn new(
        _kappaleet: &'a mut Vec<RcKappale>,
        pelihahmo: &'a mut Option<Pelihahmo>,
        _fysiikalliset: &'a mut Vec<Fysiikkakappale>,
    ) -> Self {
        TekoalyMaailma {
            _kappaleet: _kappaleet,
            pelihahmo: pelihahmo,
            _fysiikalliset: _fysiikalliset,
        }
    }
}

impl<'a> Pelihahmollinen for TekoalyMaailma<'a> {
    /// Antaa pelihahmon, jos sellainen on luotu
    fn anna_pelihahmo_mut(&mut self) -> Option<&mut Pelihahmo> {
        match &mut self.pelihahmo {
            None => None,
            Some(hahmo) => Some(hahmo),
        }
    }

    /// Antaa pelihahmon, jos sellainen on luotu
    fn anna_pelihahmo(&self) -> Option<&Pelihahmo> {
        match &self.pelihahmo {
            None => None,
            Some(hahmo) => Some(&hahmo),
        }
    }
}

impl Alyllinen {
    /// Luo uuden älyllisen kappaleen
    /// # Arguments
    /// * `kappale` - Kappale, jota tekoäly ohjaa
    /// * `aly` - Tekoälyn käyttämä äly
    pub fn new(kappale: RcKappale, aly: Box<Aly>) -> Self {
        Alyllinen {
            kappale: kappale,
            aly: aly,
        }
    }

    /// Kertoo, mitä tekoäly tekee ja palauttaa sen
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka avulla tekoäly päättää toiminnastaan
    pub fn alyile(&self, maailma: &TekoalyMaailma) -> AlyToiminta {
        self.aly.alyile(maailma, &self.kappale)
    }
}

impl Lisaosa for Alyllinen {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
    }
}

/// Tekoäly, joka osaa toimia annetun maailman perusteella
pub trait Aly {
    /// Palauttaa tekoälyn toiminnon, joka tulee annetusta pelimaailman tilasta
    /// # Arguments
    /// * `maailma` - Maailma, jonka perusteella toimitaan
    /// * `oma_kappale` Tekoälyn ohjaama kappale
    fn alyile(&self, maailma: &TekoalyMaailma, oma_kappale: &RcKappale) -> AlyToiminta;
}

/// Tekoälyn mahdolliset toimintavaihtoehdot
pub enum AlyToiminta {
    /// Ei tee mitään
    Laiskottele,
    /// Liikkuu annettuun suuntaan. Ei sisällä nopeutta
    Liiku { suunta: Vektori },
}

/// Tekoäly, joka ohjaa ohjattavan kappaleen liikkumaan suoraan pelaajan ohjaamaa hahmoa kohti
pub struct SeurausAly;

impl Aly for SeurausAly {
    /// Palauttaa tekoälyn toiminnon, joka tulee annetusta pelimaailman tilasta
    /// # Arguments
    /// * `maailma` - Maailma, jonka perusteella toimitaan
    /// * `oma_kappale` Tekoälyn ohjaama kappale
    fn alyile(&self, maailma: &TekoalyMaailma, oma_kappale: &RcKappale) -> AlyToiminta {
        if let Some(pelihahmo) = maailma.anna_pelihahmo() {
            let oma_suunta = (pelihahmo.anna_kappale().borrow().keskipisteen_sijainti()
                - oma_kappale.borrow().keskipisteen_sijainti())
            .yksikkovektori();
            return AlyToiminta::Liiku { suunta: oma_suunta };
        }
        AlyToiminta::Laiskottele
    }
}
