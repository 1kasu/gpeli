use std::cell::RefCell;
use std::rc::Rc;

use crate::fysiikka::Fysiikkakappale;
use crate::maailma::*;

type RcKappale = Rc<RefCell<Kappale>>;

pub struct Alyllinen {
    kappale: RcKappale,
    aly: Box<Aly>,
}

pub struct TekoalyMaailma<'a> {
    _kappaleet: &'a mut Vec<RcKappale>,
    pelihahmo: &'a mut Option<Pelihahmo>,
    _fysiikalliset: &'a mut Vec<Fysiikkakappale>,
}

impl<'a> TekoalyMaailma<'a> {
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
    pub fn new(kappale: RcKappale, aly: Box<Aly>) -> Self {
        Alyllinen {
            kappale: kappale,
            aly: aly,
        }
    }

    pub fn alyile(&self, maailma: &TekoalyMaailma) -> AlyToiminta {
        self.aly.alyile(maailma, &self.kappale)
    }
}

impl Lisaosa for Alyllinen {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
    }
}

pub trait Aly {
    fn alyile(&self, maailma: &TekoalyMaailma, oma_kappale: &RcKappale) -> AlyToiminta;
}

pub enum AlyToiminta {
    Laiskottele,
    Liiku { suunta: Vektori },
}

pub struct SeurausAly;

impl Aly for SeurausAly {
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
