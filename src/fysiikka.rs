use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::maailma::Kappale;
use super::maailma::Muoto;
use super::maailma::Vektori;

type RcKappale = Rc<RefCell<Kappale>>;

/// Kohde, jolle voidaan laskea fysiikkaan liittyviä laskuja.
pub trait Fysiikallinen {
    /// Antaa kohteen nopeuden
    fn anna_nopeus(&self) -> Vektori;

    /// Asettaa kohteen nopeuden
    /// # Arguments
    /// * `nopeus` - Kohteen uusi nopeus
    fn aseta_nopeus(&mut self, nopeus: Vektori);

    /// Antaa kohteen sijainnin
    fn anna_sijainti(&self) -> Vektori;

    /// Asettaa kohteen sijainnin
    /// # Arguments
    /// * `sijainti` - Kohteen uusi sijainti
    fn aseta_sijainti(&mut self, sijainti: Vektori);

    /// Antaa kohteen muodon
    fn anna_muoto(&self) -> Muoto;

    /// Laskee kohteen uuden sijainnin
    fn paivita_sijainti(&mut self, paivitysaika: &Duration);
}

pub struct Fysiikkakappale {
    /// Varsinainen kappale
    kappale: RcKappale,
    /// Kappaleen nopeus ja suunta
    nopeus: Vektori,
}

impl Fysiikkakappale {
    pub fn new(nopeus: Vektori, kappale: RcKappale) -> Self {
        Fysiikkakappale {
            kappale: kappale,
            nopeus: nopeus,
        }
    }
}

impl Fysiikallinen for Fysiikkakappale {
    /// Antaa kohteen nopeuden
    fn anna_nopeus(&self) -> Vektori {
        self.nopeus
    }

    /// Asettaa kohteen nopeuden
    /// # Arguments
    /// * `nopeus` - Kohteen uusi nopeus
    fn aseta_nopeus(&mut self, nopeus: Vektori) {
        self.nopeus = nopeus;
    }

    /// Antaa kohteen sijainnin
    fn anna_sijainti(&self) -> Vektori {
        self.kappale.borrow().sijainti
    }

    /// Asettaa kohteen sijainnin
    /// # Arguments
    /// * `sijainti` - Kohteen uusi sijainti
    fn aseta_sijainti(&mut self, sijainti: Vektori) {
        self.kappale.borrow_mut().sijainti = sijainti;
    }

    /// Antaa kohteen muodon
    fn anna_muoto(&self) -> Muoto {
        self.kappale.borrow().muoto
    }

    /// Päivittää kappaleen sijainnin annetun ajan mukaan
    /// # Arguments
    /// * `paivitysaika` - Päivityksessä käytettävä aika
    fn paivita_sijainti(&mut self, paivitysaika: &Duration) {
        if self.anna_nopeus().pituus() > 0.0 {
            let uusi_sijainti = self.anna_sijainti()
                + self.anna_nopeus() * (paivitysaika.as_micros() as f32 * 0.000_001);
            self.aseta_sijainti(uusi_sijainti);
        }
    }
}

pub struct Fysiikka;

impl Fysiikka {
    /// Laskee kaikille annetuille fysiikkakappaleille uuden sijainnin
    /// # Arguments
    /// * `kappaleet` - Päivitettävät kappaleet
    /// * `paivitysaika` - Päivityksessä käytettävä aika
    pub fn laske_uudet_sijainnit(
        &self,
        kappaleet: &mut [Fysiikkakappale],
        paivitysaika: &Duration,
    ) {
        for i in 0..kappaleet.len() {
            let vanha_sijainti = kappaleet[i].anna_sijainti();
            kappaleet[i].paivita_sijainti(paivitysaika);
            for j in 0..kappaleet.len() {
                if i == j {
                    continue;
                }
                if ovatko_paallekkain(
                    &kappaleet[i].kappale.borrow(),
                    &kappaleet[j].kappale.borrow(),
                ) {
                    kappaleet[i].aseta_sijainti(vanha_sijainti);
                    break;
                }
            }
        }
    }
}

/// Tarkistaa törmäävätkö kaksi annettua kappaletta toisiinsa
fn ovatko_paallekkain(kappale_a: &Kappale, kappale_b: &Kappale) -> bool {
    match (kappale_a.muoto, kappale_b.muoto) {
        (Muoto::Nelio(leveys_a, korkeus_a), Muoto::Nelio(leveys_b, korkeus_b)) => {
            let vasen_a = kappale_a.sijainti.x;
            let oikea_a = kappale_a.sijainti.x + leveys_a;
            let vasen_b = kappale_b.sijainti.x;
            let oikea_b = kappale_b.sijainti.x + leveys_b;
            let ala_a = kappale_a.sijainti.y;
            let yla_a = kappale_a.sijainti.y + korkeus_a;
            let ala_b = kappale_b.sijainti.y;
            let yla_b = kappale_b.sijainti.y + korkeus_b;
            !(oikea_a < vasen_b || oikea_b < vasen_a  || yla_a < ala_b || yla_b < ala_a)
        }
        (Muoto::Ympyra(sade_a), Muoto::Ympyra(sade_b)) => {
            (kappale_a.sijainti - kappale_b.sijainti).pituus() < (sade_a + sade_b)
        }
        _ => (false),
    }
}
