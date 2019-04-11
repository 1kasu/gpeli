use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::maailma::Kappale;
use super::maailma::Muoto;
use super::maailma::Sijainti;

type RcKappale = Rc<RefCell<Kappale>>;

/// Kohde, jolle voidaan laskea fysiikkaan liittyviä laskuja.
pub trait Fysiikallinen {
    /// Antaa kohteen nopeuden
    fn anna_nopeus(&self) -> Sijainti;

    /// Asettaa kohteen nopeuden
    /// # Arguments
    /// * `nopeus` - Kohteen uusi nopeus
    fn aseta_nopeus(&mut self, nopeus: Sijainti);

    /// Antaa kohteen sijainnin
    fn anna_sijainti(&self) -> Sijainti;

    /// Asettaa kohteen sijainnin
    /// # Arguments
    /// * `sijainti` - Kohteen uusi sijainti
    fn aseta_sijainti(&mut self, sijainti: Sijainti);

    /// Antaa kohteen muodon
    fn anna_muoto(&self) -> Muoto;
    
    fn paivita_sijainti(&mut self, paivitysaika: &Duration);
}


pub struct Fysiikkakappale {
    /// Varsinainen kappale
    kappale: RcKappale,
    /// Kappaleen nopeus ja suunta
    nopeus: Sijainti,
}

impl Fysiikkakappale{
    pub fn new(nopeus: Sijainti, kappale: RcKappale) -> Self{
        Fysiikkakappale{
            kappale: kappale,
            nopeus: nopeus
        }
    }
}

impl Fysiikallinen for Fysiikkakappale {
    /// Antaa kohteen nopeuden
    fn anna_nopeus(&self) -> Sijainti {
        self.nopeus
    }

    /// Asettaa kohteen nopeuden
    /// # Arguments
    /// * `nopeus` - Kohteen uusi nopeus
    fn aseta_nopeus(&mut self, nopeus: Sijainti) {
        self.nopeus = nopeus;
    }

    /// Antaa kohteen sijainnin
    fn anna_sijainti(&self) -> Sijainti {
        self.kappale.borrow().sijainti
    }

    /// Asettaa kohteen sijainnin
    /// # Arguments
    /// * `sijainti` - Kohteen uusi sijainti
    fn aseta_sijainti(&mut self, sijainti: Sijainti) {
        self.kappale.borrow_mut().sijainti = sijainti;
    }

    /// Antaa kohteen muodon
    fn anna_muoto(&self) -> Muoto {
        self.kappale.borrow().muoto
    }
    
    /// Päivittää kappaleen sijainnin annetun ajan mukaan
    /// # Arguments
    /// * `paivitysaika` - Päivityksessä käytettävä aika
    fn paivita_sijainti(&mut self, paivitysaika: &Duration){
        let uusi_sijainti = self.anna_sijainti() + self.anna_nopeus() * (paivitysaika.as_micros() as f32 * 0.000_001) ;
        self.aseta_sijainti(uusi_sijainti);
    }
}

