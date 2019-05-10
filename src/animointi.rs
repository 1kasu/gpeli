//! Sisältää animoinnissa tarvittavia luokkia

use sdl2::pixels::Color;
use std::ops::Deref;
use std::time::Duration;

use crate::maailma::kappale::Kappale;
use crate::maailma::kappale::Muoto::*;
use crate::maailma::kappale::Tagi::*;
use crate::maailma::vektori::Vektori;
use crate::paivitys::Paivitysaika;
use crate::piirtaja::{Piirtotapa, ValiaikainenPiirrettavaKappale};

type Peliaika = Duration;

/// Sisältää listan animaatioista ja animaatiuoiden muodostamista
/// piirrettävistä kappaleista. Huolehtii myös animaatioiden poistamisesta
/// kun ne kuolevat
#[derive(Default)]
pub struct Animaatiot {
    /// Lista animaatioista sisältäen tiedot kuolinajasta
    animaatiot: Vec<Kuolevainen<Box<Animaatio>>>,
}

impl Animaatiot {
    /// Luo uuden animaatioiden säilyttäjän
    pub fn new() -> Self {
        Animaatiot {
            animaatiot: Default::default(),
        }
    }

    /// Lisää uuden animaation listaan
    /// # Arguments
    /// * `animaatio` - Lisättävä animaatio
    pub fn lisaa_animaatio(&mut self, animaatio: Kuolevainen<Box<Animaatio>>) {
        self.animaatiot.push(animaatio);
    }

    /// Päivittää kaikkien animaatioiden tilaa luoden tarvittavan graafisen esityksen valmiiksi.
    /// Tarvittaessa myös tuhoaa kaikki vanhentuneet animaatiot.
    /// # Arguments
    /// * `piirrettavien_lista` - Lista, johon piirrettävät kappaleet lisätään
    /// * `pelimaailman_aika` - Kokonaisaika, joka on kulunut pelin alusta alkaen
    pub fn anna_piirrettavat(
        &mut self,
        piirrettavien_lista: &mut Vec<ValiaikainenPiirrettavaKappale>,
        pelimaailman_aika: &Paivitysaika,
    ) {
        self.animaatiot
            .retain(|x| !x.kuoleeko(pelimaailman_aika.kokonais_pelin_aika));
        for a in &mut self.animaatiot {
            if let Some(aika) = pelimaailman_aika
                .kokonais_pelin_aika
                .checked_sub(*a.animaation_alku())
            {
                a.anna_palat(piirrettavien_lista, &aika);
            }
        }
    }
}

/// Animaatio, jolta saadaan graafinen esitys antamalla ajankohta, josta muodostettava kuva halutaan
pub trait Animaatio {
    /// Lisää annettuun listaan kaikki animaation muodostamat kappaleet
    /// # Arguments
    /// * `palat` - Lista, johon animaation luomat kappaleet lisätään
    /// * `framen_aika` - Ajanhetki animaation alusta, josta muodostetaan kuva
    fn anna_palat(&self, palat: &mut Vec<ValiaikainenPiirrettavaKappale>, framen_aika: &Duration);
    /// Antaa animaation aloitushetken esittäen sen pelin käynnistymisestä kuluneessa ajasta eli kuinka
    /// paljon aikaa on kulunut pelin käynnistymisestä.
    fn animaation_alku(&self) -> &Peliaika;
}

/// Animaatio, joka animoi laatikon, joka muuttuu alkukoosta loppukooksi annetulla aikavälillä
pub struct KatoamisAnimaatio {
    /// Animaation alkuhetki
    animaation_alku: Peliaika,
    /// Animaation sijainti
    sijainti: Vektori,
    /// Laatikon koko alussa
    alkukoko: f32,
    /// Laatikon koko lopussa
    loppukoko: f32,
    /// Kuinka kauan kestää saavuttaa loppukoko
    muutoksen_kesto: Duration,
    /// Kappaleen vari
    kappaleen_vari: Color,
}

impl KatoamisAnimaatio {
    /// Luo uuden katoamisanimaation annetuilla parametreillä
    /// # Arguments
    /// * `sijainti` - Animaation sijainti
    /// * `animaation_alku` - Animaation alun ajankohta
    /// * `alkukoko` - Animoitavan kappaleen koko alussa
    /// * `loppukoko` - Animoitavan kappaleen koko lopussa
    /// * `muutoksen_kesto` - Muutoksen varattu aika
    /// * `kappaleen_vari` - Animoitavan kappaleen vari
    pub fn new(
        sijainti: Vektori,
        animaation_alku: Peliaika,
        alkukoko: f32,
        loppukoko: f32,
        muutoksen_kesto: Duration,
        kappaleen_vari: Color,
    ) -> Self {
        KatoamisAnimaatio {
            animaation_alku: animaation_alku,
            sijainti: sijainti,
            alkukoko: alkukoko,
            loppukoko: loppukoko,
            muutoksen_kesto: muutoksen_kesto,
            kappaleen_vari: kappaleen_vari,
        }
    }

    /// Antaa animaation sijainnin
    pub fn sijainti_mut(&mut self) -> &mut Vektori {
        &mut self.sijainti
    }
}

impl Animaatio for KatoamisAnimaatio {
    /// Lisää annettuun listaan kaikki animaation muodostamat kappaleet
    /// # Arguments
    /// * `palat` - Lista, johon animaation luomat kappaleet lisätään
    /// * `framen_aika` - Ajanhetki animaation alusta, josta muodostetaan kuva
    fn anna_palat(&self, palat: &mut Vec<ValiaikainenPiirrettavaKappale>, framen_aika: &Duration) {
        let frame_sekunteina = framen_aika.as_micros() as f32 / 1_000_000 as f32;
        let muutoksen_kesto_sekunteina = self.muutoksen_kesto.as_micros() as f32 / 1_000_000 as f32;
        let koko = lineaarinen_interpolaatio(
            0.0,
            self.alkukoko,
            muutoksen_kesto_sekunteina,
            self.loppukoko,
            frame_sekunteina,
        );
        //println!("{:?} {:?}", koko, frame_sekunteina);
        let a = ValiaikainenPiirrettavaKappale::new(
            Kappale::new_keskipisteella(
                Nelio(koko, koko),
                self.sijainti.x,
                self.sijainti.y,
                Partikkeli,
            ),
            Piirtotapa::Yksivarinen {
                vari: self.kappaleen_vari,
            },
        );

        palat.push(a);
    }

    /// Antaa animaation aloitushetken esittäen sen pelin käynnistymisestä kuluneessa ajasta eli kuinka
    /// paljon aikaa on kulunut pelin käynnistymisestä.
    fn animaation_alku(&self) -> &Peliaika {
        &self.animaation_alku
    }
}

/// Animaation ammusten tuhoutumiselle
pub struct AmmusAnimaatio {
    /// Animaation alkuhetki
    animaation_alku: Peliaika,
    /// Animaation sijainti
    sijainti: Vektori,
    /// Animaation suunta
    suunta: Vektori,
    /// Kuinka kauan kestää saavuttaa loppukoko
    muutoksen_kesto: Duration,
    /// Kappaleen vari
    kappaleen_vari: Color,
}

impl AmmusAnimaatio {
    /// Luo uuden katoamisanimaation annetuilla parametreillä
    /// # Arguments
    /// * `sijainti` - Animaation sijainti
    /// * `animaation_alku` - Animaation alun ajankohta
    /// * `suunta` - Animaation suunta
    /// * `muutoksen_kesto` - Muutoksen varattu aika
    /// * `kappaleen_vari` - Animoitavan kappaleen vari
    pub fn new(
        sijainti: Vektori,
        animaation_alku: Peliaika,
        suunta: Vektori,
        muutoksen_kesto: Duration,
        kappaleen_vari: Color,
    ) -> Self {
        AmmusAnimaatio {
            animaation_alku: animaation_alku,
            sijainti: sijainti,
            suunta: suunta,
            muutoksen_kesto: muutoksen_kesto,
            kappaleen_vari: kappaleen_vari,
        }
    }

    /// Antaa animaation sijainnin
    pub fn sijainti_mut(&mut self) -> &mut Vektori {
        &mut self.sijainti
    }
}

impl Animaatio for AmmusAnimaatio {
    /// Lisää annettuun listaan kaikki animaation muodostamat kappaleet
    /// # Arguments
    /// * `palat` - Lista, johon animaation luomat kappaleet lisätään
    /// * `framen_aika` - Ajanhetki animaation alusta, josta muodostetaan kuva
    fn anna_palat(&self, palat: &mut Vec<ValiaikainenPiirrettavaKappale>, framen_aika: &Duration) {
        let frame_sekunteina = framen_aika.as_micros() as f32 / 1_000_000 as f32;
        let muutoksen_kesto_sekunteina = self.muutoksen_kesto.as_micros() as f32 / 1_000_000 as f32;

        let paatos_sijainti = self.sijainti + self.suunta * 20.0;

        let sijainti = lineaarinen_interpolaatio(
            0.0,
            self.sijainti,
            muutoksen_kesto_sekunteina,
            paatos_sijainti,
            frame_sekunteina,
        );

        let koko =
            lineaarinen_interpolaatio(0.0, 10.0, muutoksen_kesto_sekunteina, 0.1, frame_sekunteina);

        let a = ValiaikainenPiirrettavaKappale::new(
            Kappale::new_keskipisteella(Nelio(koko, koko), sijainti.x, sijainti.y, Partikkeli),
            Piirtotapa::Yksivarinen {
                vari: self.kappaleen_vari,
            },
        );
        palat.push(a);
    }

    /// Antaa animaation aloitushetken esittäen sen pelin käynnistymisestä kuluneessa ajasta eli kuinka
    /// paljon aikaa on kulunut pelin käynnistymisestä.
    fn animaation_alku(&self) -> &Peliaika {
        &self.animaation_alku
    }
}

/// Sisältää jotakin joka kuolee, annettuna ajanhetkenä.
/// Ei huolehdi itse sisällön poistamisesta, mutta siltä voidaan kysyä tarvitseeko kohde jo poistaa.
pub struct Kuolevainen<T> {
    /// Ajanhetki, jolloin sisältö tulee poistaa
    kuolin_aika: Peliaika,
    /// Sisältö, joka kuuluu poistaa, kun kuolinhetki saavutetaan
    arvo: T,
}

impl<T> Kuolevainen<T> {
    /// Luo uuden kappaleen, joka kuuluu poistaa kun oikea ajanhetki saapuu
    /// # Arguments
    /// * `sisalto` - Poistettava sisältö
    /// * `kuolin_aika` - Aika, jolloin sisältö poistetaan
    pub fn new(sisalto: T, kuolin_aika: Peliaika) -> Self {
        Kuolevainen {
            arvo: sisalto,
            kuolin_aika: kuolin_aika,
        }
    }

    /// Palauttaa tiedon, kuuluisiko sisältö poistaa
    /// # Arguments
    /// * `peliaika` - Pelin ajankohta, jota verrataan kuolinaikaan
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

use std::ops::{Add, Mul, Sub};
/// Antaa lineaarisen interpolaation kahden pisteen välillä annetulla
/// interpolaatio arvolla
pub fn lineaarinen_interpolaatio<T>(
    alku_x: f32,
    alku_y: T,
    loppu_x: f32,
    loppu_y: T,
    interpolaatio_arvo: f32,
) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy,
{
    alku_y + (loppu_y - alku_y) * ((interpolaatio_arvo - alku_x) / (loppu_x - alku_x))
}
