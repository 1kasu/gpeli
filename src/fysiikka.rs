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
    /// # Arguments
    /// * `paivitysaika` - Aika, jonka verran kappaleen sijaintia päivitetään
    fn laske_uusi_sijainti(&self, paivitysaika: &Duration) -> Vektori<f32>;
}

/// Fysiikka lisäosa tavalliselle kappaleelle. Kertoo käytännössä, että
/// kappale voi törmätä ja sillä on nopeus
pub struct Fysiikkakappale {
    /// Varsinainen kappale
    kappale: RcKappale,
    /// Kappaleen nopeus ja suunta
    nopeus: Vektori,
}

impl Fysiikkakappale {
    /// Antaa uuden fysiikkakappaleen, jolla on annettu nopeus ja annettu kappale
    /// # Arguments
    /// * `nopeus` - Kappaleen alkunopeus
    /// * `kappale` - Kappale, jolle lisätään fysiikka
    pub fn new(nopeus: Vektori, kappale: RcKappale) -> Self {
        Fysiikkakappale {
            kappale: kappale,
            nopeus: nopeus,
        }
    }

    /// Antaa fysiikkakappaleen tagin
    pub fn anna_tagi(&self) -> Tagi {
        self.kappale.borrow().tagi
    }
}

impl Lisaosa for Fysiikkakappale {
    /// Antaa fysiikkakappaleen käyttämän kappaleen
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
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
        self.kappale.borrow().kulman_sijainti()
    }

    /// Asettaa kohteen sijainnin
    /// # Arguments
    /// * `sijainti` - Kohteen uusi sijainti
    fn aseta_sijainti(&mut self, sijainti: Vektori) {
        self.kappale.borrow_mut().aseta_kulman_sijainti(sijainti);
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

/// Sisältää listan kaikista tapahtuneista törmäyksistä. Perustuu indekseihin, joten
/// ei ole pitkään ajankohtainen.
#[derive(Default)]
pub struct Tormaystiedot {
    /// Lista törmäyksistä ja niiden tiedoista
    tormays_tiedot: Vec<Tormaystieto>,
}

impl Tormaystiedot {
    /// Antaa uuden törmäystiedot otuksen
    pub fn new() -> Self {
        Tormaystiedot {
            tormays_tiedot: Vec::new(),
        }
    }

    /// Lisää törmäyksen tägin törmäystietoon ja tarvittaessa luo törmäystiedon
    /// # Arguments
    /// * `indeksi` - törmänneen kappaleen indeksi
    /// * `tagi` - Törmätyn kappaleen tagi
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

/// Sisältää yksittäisen kappaleen tiedot, että mihin kaikkiin tageihin
/// on törmännyt.
pub struct Tormaystieto {
    /// Törmänneen kappaleen indeksi
    pub indeksi: usize,
    /// Lista kohteiden tageista, joihin on törmätty
    tormatyt_kohteet: Vec<Tagi>,
}

impl Tormaystieto {
    /// Luo uuden törmäystiedon annetulla indeksillä
    /// # Arguments
    /// * `indeksi` - Törmääjän indeksi
    pub fn new(indeksi: usize) -> Self {
        Tormaystieto {
            indeksi: indeksi,
            tormatyt_kohteet: Vec::new(),
        }
    }

    /// Lisää annetun tagin, jos kyseinen tagi ei jo ole lisätty
    /// # Arguments
    /// * `lisattava_tagi` - Törmätyn kohteen tagi
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

/// Fysiikka otus, joka muistaa jotakin fysiikan päivitysksistä
#[derive(Default)]
pub struct Fysiikka {
    /// Viimeisimmän fysiikkapäivityksen aikana tapahtuneet törmäykset
    pub tormaykset: Tormaystiedot,
}

impl Fysiikka {
    /// Luo uuden fysiikan
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
/// # Arguments
/// * `kappale_a` - Kappale, joka ns törmää
/// * `kappale_b` - Kappale, joka ns tulee törmätyksi
fn ovatko_paallekkain(kappale_a: &Kappale, kappale_b: &Kappale) -> bool {
    match (kappale_a.muoto, kappale_b.muoto) {
        (Muoto::Nelio(leveys_a, korkeus_a), Muoto::Nelio(leveys_b, korkeus_b)) => {
            let sijainti_a = kappale_a.kulman_sijainti();
            let sijainti_b = kappale_b.kulman_sijainti();

            let vasen_a = sijainti_a.x;
            let oikea_a = sijainti_a.x + leveys_a;
            let vasen_b = sijainti_b.x;
            let oikea_b = sijainti_b.x + leveys_b;
            let ala_a = sijainti_a.y;
            let yla_a = sijainti_a.y + korkeus_a;
            let ala_b = sijainti_b.y;
            let yla_b = sijainti_b.y + korkeus_b;
            !(oikea_a < vasen_b || oikea_b < vasen_a || yla_a < ala_b || yla_b < ala_a)
        }
        (Muoto::Ympyra(sade_a), Muoto::Ympyra(sade_b)) => {
            (kappale_a.keskipisteen_sijainti() - kappale_b.keskipisteen_sijainti()).pituus()
                < (sade_a + sade_b)
        }
        (Muoto::Ympyra(_), Muoto::Nelio(_, _)) => ympyran_ja_nelion_tormays(kappale_b, kappale_a),
        (Muoto::Nelio(_, _), Muoto::Ympyra(_)) => ympyran_ja_nelion_tormays(kappale_a, kappale_b),
    }
}

/// Tarkistaa ovatko annetut neliö ja ympyrä toistensa päällä.
/// Antaa false, jos oletut muodot(ensin neliö ja sitten ympyrä) eivät päde.
/// # Arguments
/// * `nelio` - Pitää olla neliö tai false
/// * `ympyra` - Pitää olla ympyrä tai false
fn ympyran_ja_nelion_tormays(nelio: &Kappale, ympyra: &Kappale) -> bool {
    if let Muoto::Nelio(leveys, korkeus) = nelio.muoto {
        if let Muoto::Ympyra(sade) = ympyra.muoto {
            let ympyra_sijainti = ympyra.keskipisteen_sijainti();
            let nelio_sijainti = nelio.kulman_sijainti();
            let vasen = nelio_sijainti.x;
            let oikea = nelio_sijainti.x + leveys;
            let ala = nelio_sijainti.y;
            let yla = nelio_sijainti.y + korkeus;
            let vasen_yla_kulma = Vektori::new(vasen, yla);
            let oikea_yla_kulma = Vektori::new(oikea, yla);
            let vasen_ala_kulma = Vektori::new(vasen, ala);
            let oikea_ala_kulma = Vektori::new(oikea, ala);

            return !(
                // Rajataan neliö, jonka ulkopuolella törmäys ei voi tapahtua
                ympyra_sijainti.x <= vasen - sade
                || oikea + sade <= ympyra_sijainti.x
                || ympyra_sijainti.y <= ala - sade
                || yla + sade <= ympyra_sijainti.y
                // Tarkistetaan, onko se, neliön kulman muodostaman ympyrän ulkopuolella
                || (ympyra_sijainti - vasen_yla_kulma).pituus() >= sade
                    && ympyra_sijainti.x < vasen
                    && ympyra_sijainti.y > yla
                || (ympyra_sijainti - oikea_yla_kulma).pituus() >= sade
                    && ympyra_sijainti.x > oikea
                    && ympyra_sijainti.y > yla
                || (ympyra_sijainti - vasen_ala_kulma).pituus() >= sade
                    && ympyra_sijainti.x < vasen
                    && ympyra_sijainti.y < ala
                || (ympyra_sijainti - oikea_ala_kulma).pituus() >= sade
                    && ympyra_sijainti.x > oikea
                    && ympyra_sijainti.y < ala
            );
        }
    }
    false
}
