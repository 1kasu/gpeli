use std::cell::RefCell;
use std::rc::Rc;

use crate::animointi::{lineaarinen_interpolaatio, Animaatiot};
use crate::fysiikka::{Fysiikallinen, Fysiikkakappale};
use crate::piirtaja::{PiirrettavaKappale, PiirrettavaMaailma, ValiaikainenPiirrettavaKappale};
use crate::tekoaly::{AlyToiminta, Alyllinen, TekoalyMaailma};
use kappale::Kappale;
use kappalemuisti::Kappalemuisti;
use pelihahmo::Pelihahmo;
use vektori::Vektori;

pub mod kappale;
pub mod kappalemuisti;
pub mod pelihahmo;
pub mod vektori;

pub type RcKappale = Rc<RefCell<Kappale>>;

/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
#[derive(Default)]
pub struct Perusmaailma {
    /// Pelimaailman sisältämät kappaleet
    kappaleet: Vec<RcKappale>,
    /// Muistit vanhoista tiloista
    kappalemuisti: Vec<Kappalemuisti>,
    /// Jos jotakin, niin interpolointi on käytössä
    /// Kuvastaa arvoa, jolla seuraava piirtäminen tehdään
    interpoloinnin_arvo: Option<f32>,
    /// Maailmassa olevat fysiikkakappaleet
    fysiikka_kappaleet: Vec<Fysiikkakappale>,
    /// Piirrettävät kappaleet
    piirrettavat_kappaleet: Vec<PiirrettavaKappale>,
    /// Mahdollinen pelattava hahmo
    pelihahmo: Option<Pelihahmo>,
    /// Sisältää kaikki tekoälyä käyttävät otukset
    alylliset: Vec<Alyllinen>,
    /// Poistettavat kappaleet
    poistettavat: Vec<RcKappale>,
    /// Lista kaikista animaatioista
    pub animaatiot: Animaatiot,
}

impl Perusmaailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Perusmaailma {
            kappaleet: Vec::new(),
            kappalemuisti: Default::default(),
            interpoloinnin_arvo: None,
            fysiikka_kappaleet: Vec::new(),
            piirrettavat_kappaleet: Vec::new(),
            alylliset: Default::default(),
            pelihahmo: None,
            poistettavat: Vec::new(),
            animaatiot: Default::default(),
        }
    }

    /// Luo uuden tyhjän maailman
    pub fn new_interpoloiva() -> Self {
        Perusmaailma {
            kappaleet: Vec::new(),
            kappalemuisti: Default::default(),
            interpoloinnin_arvo: Some(0.0),
            fysiikka_kappaleet: Vec::new(),
            piirrettavat_kappaleet: Vec::new(),
            alylliset: Default::default(),
            pelihahmo: None,
            poistettavat: Vec::new(),
            animaatiot: Default::default(),
        }
    }

    /// Lisää annetun kappaleen maailmaan ja antaa viiteen siihen
    /// # Arguments
    /// * `kappale` - Lisättävä kappale
    pub fn lisaa_kappale(&mut self, kappale: Kappale) -> RcKappale {
        let r_kappale = Rc::new(RefCell::new(kappale));
        self.kappaleet.push(Rc::clone(&r_kappale));
        // Luo kappaleelle muistin, jos interpolointi on käytössä
        if self.interpoloinnin_arvo.is_some() {
            self.kappalemuisti
                .push(Kappalemuisti::new(Rc::clone(&r_kappale)));
        }
        r_kappale
    }

    pub fn aseta_interpolaatio_arvo(&mut self, arvo: f32) {
        self.interpoloinnin_arvo = Some(arvo);
    }

    pub fn paivita_kappalemuistia(&mut self) {
        for kappale in &mut self.kappalemuisti {
            kappale.paivita_muistia();
        }
    }

    /// Lisää annetulle kappaleelle piirrettävyys ominaisuuden
    /// # Arguments
    /// * `kappale` - Lisättävä piirrettava kappale
    pub fn lisaa_piirrettava_kappale(&mut self, kappale: PiirrettavaKappale) {
        self.piirrettavat_kappaleet.push(kappale);
    }

    /// Lisää annettavalle kappaleelle fysiikan
    /// # Arguments
    /// * `kappale` - Lisättävä fysiikkakappale
    pub fn lisaa_fysiikkakappale(&mut self, kappale: Fysiikkakappale) {
        self.fysiikka_kappaleet.push(kappale);
    }

    /// Tekee annetusta kappaleesta pelihahmon
    pub fn lisaa_pelihahmo(&mut self, pelihahmo: Pelihahmo) {
        if self.pelihahmo.is_none() {
            self.pelihahmo = Some(pelihahmo);
        }
    }

    /// Lisää tekoälyn maailmaan
    /// # Arguments
    /// * `alyllinen` - Lisättävä tekoäly
    pub fn lisaa_aly(&mut self, alyllinen: Alyllinen) {
        self.alylliset.push(alyllinen);
    }

    /// Onko maailmassa pelihahmo olemassa
    pub fn onko_pelihahmo(&self) -> bool {
        self.pelihahmo.is_some()
    }

    /// Antaa fysiikkalliset kappaleet
    pub fn fysiikalliset(&mut self) -> &mut [Fysiikkakappale] {
        &mut self.fysiikka_kappaleet
    }

    /// Antaa kaikki ai-hahmot
    pub fn alylliset(&mut self) -> &mut [Alyllinen] {
        &mut self.alylliset
    }

    /// Toteuttaa kaikkien tekoälyjen toiminnot
    pub fn laske_tekoalyt(&mut self) {
        let tmaailma = TekoalyMaailma::new(
            &mut self.kappaleet,
            &mut self.pelihahmo,
            &mut self.fysiikka_kappaleet,
        );
        let mut toiminnot = Vec::new();
        for aly in &self.alylliset {
            toiminnot.push(aly.alyile(&tmaailma));
        }

        for (indeksi, toiminto) in toiminnot.iter().enumerate() {
            match toiminto {
                AlyToiminta::Laiskottele => (),
                AlyToiminta::Liiku { suunta } => {
                    if let Some(f_kappale) =
                        self.anna_fysiikka_mut(&self.alylliset[indeksi].anna_kappale())
                    {
                        f_kappale.aseta_nopeus(*suunta * 40.0);
                    }
                }
            }
        }
    }

    /// Lisää kappaleen poistettavien kappaleiden listaan.
    /// # Arguments
    /// * `poistettava` - Kappale, joka merkitään poistettavaksi
    pub fn lisaa_poistettava(&mut self, poistettava: RcKappale) {
        self.poistettavat.push(poistettava);
    }

    /// Poistaa poistettaviksi merkityt kappaleet kappaleisiin viittaavien ominaisuuksien kanssa
    pub fn poista_poistettavat(&mut self) {
        while let Some(poistettava) = self.poistettavat.pop() {
            // Poistaa kappaleen fysiikkakappaleista
            self.fysiikka_kappaleet
                .retain(|x| !std::ptr::eq(x.anna_kappale().as_ptr(), poistettava.as_ptr()));
            // Poistaa kappaleen piirrettävistä
            self.piirrettavat_kappaleet
                .retain(|x| !std::ptr::eq(x.anna_kappale().as_ptr(), poistettava.as_ptr()));
            // poisttaa tekoälyn
            self.alylliset
                .retain(|x| !std::ptr::eq(x.anna_kappale().as_ptr(), poistettava.as_ptr()));
            // poistaa muistin
            self.kappalemuisti
                .retain(|x| !std::ptr::eq(x.anna_kappale().as_ptr(), poistettava.as_ptr()));
            // Poistaa kappaleen kappaleista
            self.kappaleet
                .retain(|x| !std::ptr::eq(x.as_ptr(), poistettava.as_ptr()));
            // Poistaa kappaleen pelihahmosta
            if let Some(hahmo) = &mut self.pelihahmo {
                if std::ptr::eq(hahmo.anna_kappale().as_ptr(), poistettava.as_ptr()) {
                    self.pelihahmo = None
                }
            }
        }
    }

    /// Antaa piirrettävät kappaleet
    pub fn anna_piirrettavat(&self, lista: &mut Vec<ValiaikainenPiirrettavaKappale>) {
        if let Some(interpolaatio_arvo) = self.interpoloinnin_arvo {
            for kappale in &self.kappalemuisti {
                let (vasen, oikea) = kappale.anna_versiot();

                let interpoloitu_sijainti = lineaarinen_interpolaatio(
                    0.0,
                    vasen.kulman_sijainti(),
                    1.0,
                    oikea.kulman_sijainti(),
                    interpolaatio_arvo,
                );
                let kappale = ValiaikainenPiirrettavaKappale::new(
                    Kappale::new_kulmalla(
                        oikea.muoto,
                        interpoloitu_sijainti.x,
                        interpoloitu_sijainti.y,
                        vasen.tagi,
                    ),
                    (self
                        .anna_piirrettavyys(&kappale.anna_kappale())
                        .unwrap()
                        .anna_piirtotapa())
                    .clone(),
                );

                lista.push(kappale);
            }
        }
    }
}

pub trait Pelihahmollinen {
    /// Antaa pelihahmon, jos sellainen on luotu
    fn anna_pelihahmo_mut(&mut self) -> Option<&mut Pelihahmo>;

    /// Antaa pelihahmon, jos sellainen on luotu
    fn anna_pelihahmo(&self) -> Option<&Pelihahmo>;
}

impl Pelihahmollinen for Perusmaailma {
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

impl LisaosienAntaja for Perusmaailma {
    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys_mut(&mut self, kappale: &RcKappale) -> Option<&mut PiirrettavaKappale> {
        for piirrettava in &mut self.piirrettavat_kappaleet {
            if std::ptr::eq(piirrettava.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(piirrettava);
            }
        }
        None
    }

    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys(&self, kappale: &RcKappale) -> Option<&PiirrettavaKappale> {
        for piirrettava in &self.piirrettavat_kappaleet {
            if std::ptr::eq(piirrettava.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(piirrettava);
            }
        }
        None
    }

    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka_mut(&mut self, kappale: &RcKappale) -> Option<&mut Fysiikkakappale> {
        for fysiikka in &mut self.fysiikka_kappaleet {
            if std::ptr::eq(fysiikka.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(fysiikka);
            }
        }
        None
    }

    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka(&self, kappale: &RcKappale) -> Option<&Fysiikkakappale> {
        for fysiikka in &self.fysiikka_kappaleet {
            if std::ptr::eq(fysiikka.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(fysiikka);
            }
        }
        None
    }
}

impl PiirrettavaMaailma for Perusmaailma {
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
        match self.anna_pelihahmo() {
            None => None,
            Some(hahmo) => Some(hahmo.anna_kappale().borrow().keskipisteen_sijainti()),
        }
    }
}

/// Pystyy antamaan annetun kappaleen lisäosat pyydettäessä
pub trait LisaosienAntaja {
    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys_mut(&mut self, kappale: &RcKappale) -> Option<&mut PiirrettavaKappale>;
    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys(&self, kappale: &RcKappale) -> Option<&PiirrettavaKappale>;
    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka_mut(&mut self, kappale: &RcKappale) -> Option<&mut Fysiikkakappale>;
    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka(&self, kappale: &RcKappale) -> Option<&Fysiikkakappale>;
}

/// Tulee toteuttaa, jos laajentaa peruskappaleen toiminnallisuutta esim. pirrettäessä
/// Tarvitaan esim. kun kappale poistetaan, jolloin myös kappaleeseen liitetyt lisäosat
/// poistetaan.
pub trait Lisaosa {
    /// Antaa lisäosan käyttämään kappaleeseen kopiodun viitteen
    fn anna_kappale(&self) -> RcKappale;
}
