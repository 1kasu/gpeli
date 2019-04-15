use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::maailma::Kappale;
use super::maailma::Muoto;
use super::maailma::Tagi;
use super::maailma::Vektori;
use crate::maailma::Lisaosa;

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

    /// Laskee kohteen uuden sijainnin ja palauttaa sen
    fn laske_uusi_sijainti(&self, paivitysaika: &Duration) -> Vektori<f32>;
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



    pub fn anna_tagi(&self) -> Tagi {
        self.kappale.borrow().tagi
    }
}

impl Lisaosa for Fysiikkakappale {
    fn anna_kappale(&self) -> &RcKappale {
        &self.kappale
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
    fn laske_uusi_sijainti(&self, paivitysaika: &Duration) -> Vektori<f32> {
        self.anna_sijainti() + self.anna_nopeus() * (paivitysaika.as_micros() as f32 * 0.000_001)
    }
}

#[derive(Default)]
pub struct Tormaystiedot {
    tormays_tiedot: Vec<Tormaystieto>,
}

impl Tormaystiedot {
    pub fn new() -> Self {
        Tormaystiedot {
            tormays_tiedot: Vec::new(),
        }
    }

    /// Lisää törmäyksen tägin törmäystietoon ja tarvittaessa luo törmäystiedon
    pub fn lisaa_tormays(&mut self, indeksi: usize, tagi: Tagi) {
        match self
            .tormays_tiedot
            .iter_mut()
            .find(|x| x.indeksi == indeksi)
        {
            Some(a) => a.lisaa_tagi(tagi),
            None => {
                let mut uusi_tormaystieto = Tormaystieto::new(indeksi);
                uusi_tormaystieto.lisaa_tagi(tagi);
                self.tormays_tiedot.push(uusi_tormaystieto);
            }
        }
    }

    /// Antaa tiedot kaikista kerätyistä törmäyksistä
    pub fn anna_tormaykset(&self) -> &[Tormaystieto] {
        &self.tormays_tiedot
    }
}

pub struct Tormaystieto {
    pub indeksi: usize,
    tormatyt_kohteet: Vec<Tagi>,
}

impl Tormaystieto {
    pub fn new(indeksi: usize) -> Self {
        Tormaystieto {
            indeksi: indeksi,
            tormatyt_kohteet: Vec::new(),
        }
    }

    /// Lisää annetun tagin
    pub fn lisaa_tagi(&mut self, lisattava_tagi: Tagi) {
        if !self.tormatyt_kohteet.contains(&lisattava_tagi) {
            self.tormatyt_kohteet.push(lisattava_tagi)
        }
    }

    /// Antaa törmättyjen kohteiden tagit
    pub fn anna_tagit(&self) -> &[Tagi] {
        &self.tormatyt_kohteet
    }
}

pub struct Fysiikka {
    pub tormaykset: Tormaystiedot,
}
impl Fysiikka {
    pub fn new() -> Self {
        Fysiikka {
            tormaykset: Default::default(),
        }
    }
}

impl Fysiikka {
    /// Laskee kaikille annetuille fysiikkakappaleille uuden sijainnin ja palauttaa tiedot tapahtuneista törmäyksistä
    /// # Arguments
    /// * `kappaleet` - Päivitettävät kappaleet
    /// * `paivitysaika` - Päivityksessä käytettävä aika
    pub fn laske_uudet_sijainnit(
        &mut self,
        kappaleet: &mut [Fysiikkakappale],
        paivitysaika: &Duration,
    ) {
        let mut vanhat_sijainnit = Vec::new();
        self.tormaykset = Tormaystiedot::new();

        // Laskee uudet sijainnit
        for kappale in kappaleet.iter_mut() {
            vanhat_sijainnit.push(kappale.anna_sijainti());
            kappale.aseta_sijainti(kappale.laske_uusi_sijainti(paivitysaika));
        }

        // Tarkistetaan törmäykset uusien sijaintien välillä
        for i in 0..kappaleet.len() {
            // Tarkistaa aiheuttaako se törmäyksen
            for j in 0..kappaleet.len() {
                if i == j {
                    // Törmäys itsensä kannssa ei ole järkevä luonnollisestikaan
                    continue;
                }
                if ovatko_paallekkain(
                    &kappaleet[i].kappale.borrow(),
                    &kappaleet[j].kappale.borrow(),
                ) {
                    // Törmäys tapahtuu
                    // Merkitään törmäys muistiin
                    self.tormaykset
                        .lisaa_tormays(i, kappaleet[j].kappale.borrow().tagi)
                }
            }
        }

        // Perutaan kaikkien törmänneiden liike
        for tormays in self.tormaykset.anna_tormaykset() {
            let tormaajan_indeksi = tormays.indeksi;
            kappaleet[tormaajan_indeksi].aseta_sijainti(vanhat_sijainnit[tormaajan_indeksi]);
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
            !(oikea_a < vasen_b || oikea_b < vasen_a || yla_a < ala_b || yla_b < ala_a)
        }
        (Muoto::Ympyra(sade_a), Muoto::Ympyra(sade_b)) => {
            (kappale_a.sijainti - kappale_b.sijainti).pituus() < (sade_a + sade_b)
        }
        _ => (false),
    }
}
