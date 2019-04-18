use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

use crate::fysiikka::Fysiikkakappale;
use crate::piirtaja::PiirrettavaKappale;

type RcKappale = Rc<RefCell<Kappale>>;

pub trait LisaosienAntaja {
    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys(&mut self, kappale: &RcKappale) -> Option<&mut PiirrettavaKappale>;
    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka(&mut self, kappale: &RcKappale) -> Option<&mut Fysiikkakappale>;
}

pub trait PiirrettavaMaailma {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat(&self, sijainti: Vektori) -> &[PiirrettavaKappale];

    /// Antaa kameran sijainnin pelimaailmassa, jos maailma haluaa ehdottaa jotakin
    fn anna_kameran_sijainti(&self) -> Option<Vektori>;
}

/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
#[derive(Default)]
pub struct Perusmaailma {
    /// Pelimaailman sisältämät kappaleet
    kappaleet: Vec<RcKappale>,
    /// Maailmassa olevat fysiikkakappaleet
    fysiikka_kappaleet: Vec<Fysiikkakappale>,
    /// Piirrettävät kappaleet
    piirrettavat_kappaleet: Vec<PiirrettavaKappale>,
    /// Mahdollinen pelattava hahmo
    pelihahmo: Option<Pelihahmo>,
    /// Poistettavat kappaleet
    poistettavat: Vec<RcKappale>,
}

/// Pelissä oleva pelaajan ohjaama hahmo. Esimerkiksi kamera seuraa automaattisesti tätä.
pub struct Pelihahmo {
    /// Pelihahmon käyttämä kappale
    kappale: RcKappale,
    /// Pelihahmon viimeisin suunta esim. minne katsoo, ampuu jne
    suunta: Vektori<f32>,
}

impl Pelihahmo {
    // Lisää annetun kappaleen pelihahmoksi.
    pub fn new(kappale: RcKappale) -> Self {
        Pelihahmo {
            kappale: kappale,
            suunta: Vektori::new(1.0, 0.0).yksikkovektori(),
        }
    }

    /// Antaa pelihahmon suunnan
    pub fn anna_suunta(&self) -> Vektori<f32> {
        self.suunta
    }

    /// Asettaa uuden suunnan pelihahmolle
    /// # Arguments
    /// * `suunta` - Pelihahmon uusi suunta.
    pub fn aseta_suunta(&mut self, suunta: Vektori<f32>) {
        let yksikkovektori = suunta.yksikkovektori();
        if yksikkovektori.x.is_finite() && yksikkovektori.y.is_finite() {
            self.suunta = yksikkovektori;
        }
    }
}

impl Lisaosa for Pelihahmo {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
    }
}

impl Perusmaailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Perusmaailma {
            kappaleet: Vec::new(),
            fysiikka_kappaleet: Vec::new(),
            piirrettavat_kappaleet: Vec::new(),
            pelihahmo: None,
            poistettavat: Vec::new(),
        }
    }

    /// Lisää annetun kappaleen maailmaan ja antaa viiteen siihen
    /// # Arguments
    /// * `kappale` - Lisättävä kappale
    pub fn lisaa_kappale(&mut self, kappale: Kappale) -> RcKappale {
        let r_kappale = Rc::new(RefCell::new(kappale));
        self.kappaleet.push(Rc::clone(&r_kappale));
        r_kappale
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

    /// Antaa pelihahmon, jos sellainen on luotu
    pub fn anna_pelihahmo_mut(&mut self) -> Option<&mut Pelihahmo> {
        match &mut self.pelihahmo {
            None => None,
            Some(hahmo) => Some(hahmo),
        }
    }

    /// Antaa pelihahmon, jos sellainen on luotu
    pub fn anna_pelihahmo(&self) -> Option<&Pelihahmo> {
        match &self.pelihahmo {
            None => None,
            Some(hahmo) => Some(&hahmo),
        }
    }

    /// Onko maailmassa pelihahmo olemassa
    pub fn onko_pelihahmo(&self) -> bool {
        self.pelihahmo.is_some()
    }

    /// Antaa fysiikkalliset kappaleet
    pub fn fysiikalliset(&mut self) -> &mut [Fysiikkakappale] {
        &mut self.fysiikka_kappaleet
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
}

impl LisaosienAntaja for Perusmaailma {
    /// Antaa annettuun kappaleeseen liitetyt piirto-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka piirto-ominaisuutta pyydetään
    fn anna_piirrettavyys(&mut self, kappale: &RcKappale) -> Option<&mut PiirrettavaKappale> {
        for piirrettava in &mut self.piirrettavat_kappaleet {
            if std::ptr::eq(piirrettava.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(piirrettava);
            }
        }
        None
    }

    /// Antaa annettuun kappaleeseen liitetyt fysiikka-ominaisuudet, jos niitä on
    /// # Arguments
    /// * `kappale` - Kappale, jonka fysiikka-ominaisuutta pyydetään
    fn anna_fysiikka(&mut self, kappale: &RcKappale) -> Option<&mut Fysiikkakappale> {
        for fysiikka in &mut self.fysiikka_kappaleet {
            if std::ptr::eq(fysiikka.anna_kappale().as_ptr(), kappale.as_ptr()) {
                return Some(fysiikka);
            }
        }
        None
    }
}

/// Tulee toteuttaa, jos laajentaa peruskappaleen toiminnallisuutta esim. pirrettäessä
/// Tarvitaan esim. kun kappale poistetaan, jolloin myös kappaleeseen liitetyt lisäosat
/// poistetaan.
pub trait Lisaosa {
    /// Antaa lisäosan käyttämään kappaleeseen kopiodun viitteen
    fn anna_kappale(&self) -> RcKappale;
}

impl PiirrettavaMaailma for Perusmaailma {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat(&self, _sijainti: Vektori) -> &[PiirrettavaKappale] {
        &self.piirrettavat_kappaleet
    }

    /// Antaa kameran sijainnin pelimaailmassa, jos maailma haluaa ehdottaa jotakin
    fn anna_kameran_sijainti(&self) -> Option<Vektori> {
        match self.anna_pelihahmo() {
            None => None,
            Some(hahmo) => Some(hahmo.kappale.borrow().sijainti),
        }
    }
}

/// Sijainti 2d maailmassa. Muodoilla vasemman yläkulman sijainti. Origo on vasemmassa yläkulmassa.
#[derive(Copy, Clone)]
pub struct Vektori<T = f32> {
    /// x-koordinaatti
    pub x: T,
    /// y-koordinaatti
    pub y: T,
}

impl<T> Vektori<T> {
    /// Luo uuden sijainnin
    /// # Arguments
    /// * `x` - sijainnin x-koordinaatti
    /// * `y` - sijainnin y-koordinaatti
    pub fn new(x: T, y: T) -> Self {
        Vektori { x, y }
    }
}

impl<T> Default for Vektori<T>
where
    T: Default,
{
    fn default() -> Self {
        Vektori {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: std::ops::AddAssign> Vektori<T> {
    /// Siirtää sijaintia annetun verran
    /// # Arguments
    /// * `x` - x-koordinaatin muutos
    /// * 'y` - y-koordinaatin muutos
    pub fn liiku(&mut self, x: T, y: T) {
        self.x += x;
        self.y += y;
    }
}

// Muhahahaa!!!
// Vaviskaa maan matoset!
// Muutaman tunnin jälkeen vihdoin sain kirjoitettua tämän oikein!
// Nyt voin käyttää *-operaattoria sijainnille millä tahansa kertolaskua
// tukevalla tyypillä.
impl<T> Mul<T> for Vektori<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vektori<T>;
    fn mul(self, other: T) -> Self::Output {
        Vektori {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Div<T> for Vektori<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vektori<T>;
    fn div(self, other: T) -> Self::Output {
        Vektori {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: Add<Output = T>> Add for Vektori<T> {
    type Output = Vektori<T>;
    fn add(self, other: Self::Output) -> Self::Output {
        Vektori {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vektori<T> {
    type Output = Vektori<T>;

    fn sub(self, other: Self::Output) -> Self::Output {
        Vektori {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vektori<f32> {
    /// Antaa annetun vektorin pituuden
    pub fn pituus(self) -> f32 {
        // Näyttää kaamealta...
        (f32::powf(self.x, 2.0) + f32::powf(self.y, 2.0)).sqrt()
    }

    /// Antaa annetun vektorin yksikkövektorin
    pub fn yksikkovektori(self) -> Self {
        self / self.pituus()
    }
}

/// Ennalta määrätty muoto kuten neliä tai ympyrä
#[derive(Copy, Clone)]
pub enum Muoto {
    /// Tarkkaan ottaen suorakaide, jolla on leveys ja korkeus
    Nelio(f32, f32),
    /// Ympyrä, jolla on säde
    Ympyra(f32),
}

/// Kertoo minkälainen kappale on kyseessä.
#[derive(PartialEq, Copy, Clone)]
pub enum Tagi {
    Vihollinen,
    Seina,
    Ammus,
    Pelaaja,
}

/// Kappale, jolla on muoto ja sijainti
pub struct Kappale {
    /// Kappaleen muoto
    pub muoto: Muoto,
    /// Kappaleen sijainti
    pub sijainti: Vektori<f32>,
    /// Minkälainen kappale on kyseessä
    pub tagi: Tagi,
}

impl Kappale {
    /// Luo uuden kappaleen
    /// # Arguments
    /// * `muoto` - Kappaleen muoto
    /// * `x` - Kappaleen keskipisteen sijainnin x-koordinaatti
    /// * `y` - Kappaleen keskipisteen sijainnin y-koordinaatti
    pub fn new(muoto: Muoto, x: f32, y: f32, tagi: Tagi) -> Self {
        match muoto {
            Muoto::Nelio(xl, yl) => Kappale {
                muoto: muoto,
                sijainti: Vektori::new(x - xl / 2.0, y - yl / 2.0),
                tagi: tagi,
            },
            Muoto::Ympyra(r) => Kappale {
                muoto: muoto,
                sijainti: Vektori::new(x - r, y - r),
                tagi: tagi,
            },
        }
    }
}
