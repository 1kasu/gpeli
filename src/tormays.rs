use crate::fysiikka::Tormaystieto;
use crate::maailma::kappale::Tagi;
use crate::maailma::*;
use crate::paivitys::Paivitysaika;

/// Törmäystoiminta, joka tehdään, jos törmääjällä on oikea tägi.
pub trait Tormaystoiminta {
    /// Koskeeko törmäystapahtuma annettau tägiä
    fn ehto(&self, tagi: Tagi) -> bool;
    /// Toiminta, joka tehdään ehdon toteutuessa.
    /// # Arguments
    /// * `tormays` - Törmäyksen tiedot
    /// * `maailma` - Pelimaailma, jossa törmäys tapahtuu
    /// * `paivitysaika` - Päivitysaika
    fn toiminta(&self, tormays: &Tormaystieto, maailma: &mut Perusmaailma, paivitysaika: &Paivitysaika);
}

/// Törmäys tapahtuma, jolle voidaan asettaa ehdoksi törmääjän mahdolliset tägit
/// ja törmätyn mahdolliset tägit. Parametrinä annetaan myös törmäyksestä seuraavan
/// tapahtuman funktio.
pub struct YleinenTormays<'a> {
    /// Törmääjän tägit
    omat_tagit: Vec<Tagi>,
    /// Törmätyn tägit
    kohteen_tagit: Vec<Tagi>,
    /// Tapahtuma, jota kutsutaan, jos ehdot toteutuvat
    tapahtuma: &'a Fn(&Tormaystieto, &mut Perusmaailma, &Paivitysaika),
}

impl<'a> YleinenTormays<'a> {
    /// Luo uuden törmäystapahtuman
    /// # Arguments
    /// * `omat_tagit` - Lista törmääjän tageista, joita törmäystapahtuma koskee
    /// * `kohteiden_tagit` - Lista törmätyn kohteen tageista, jotka vaaditaa törmäystapahtumaa varten
    /// * `tapahtuma` - Funktio, jota kutsutaan, jos törmääjä ja törmätty vastaavat annettuja tageja
    pub fn new(
        omat_tagit: Vec<Tagi>,
        kohteiden_tagit: Vec<Tagi>,
        tapahtuma: &'a Fn(&Tormaystieto, &mut Perusmaailma, &Paivitysaika),
    ) -> Self {
        YleinenTormays {
            omat_tagit: omat_tagit,
            kohteen_tagit: kohteiden_tagit,
            tapahtuma,
        }
    }
}

impl<'a> Tormaystoiminta for YleinenTormays<'a> {
    /// Koskeeko törmäystapahtuma annettau tägiä
    fn ehto(&self, tagi: Tagi) -> bool {
        self.omat_tagit.contains(&tagi)
    }

    /// Toiminta, joka tehdään ehdon toteutuessa. Tarkistaa vielä onko törmäyksen kohde
    /// oikea ennen varsinaista törmäysfunktion kutsua
    /// # Arguments
    /// * `tormays` - Törmäyksen tiedot
    /// * `maailma` - Pelimaailma, jossa törmäys tapahtuu
    /// * `paivitysaika` - Päivitysaika
    fn toiminta(&self, tormays: &Tormaystieto, maailma: &mut Perusmaailma, paivitysaika: &Paivitysaika) {
        // Katstaan onko mikään törmätyn kohteen tageista haluttujen joukossa
        if self
            .kohteen_tagit
            .iter()
            .skip_while(|x| !tormays.anna_tagit().contains(x))
            .next()
            .is_some()
        {
            (self.tapahtuma)(tormays, maailma, paivitysaika);
        }
    }
}
