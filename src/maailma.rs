use std::cell::RefCell;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

use super::fysiikka::Fysiikkakappale;

type RcKappale = Rc<RefCell<Kappale>>;

pub trait PiirrettavaMaailma {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat(&self, sijainti: Vektori) -> &[RcKappale];

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
    /// Onko pelihahmo luotu jo
    pelihahmo: Option<Pelihahmo>,
}

pub struct Pelihahmo {
    pub kappale: RcKappale,
}

impl Pelihahmo {
    pub fn new(kappale: RcKappale) -> Self {
        Pelihahmo { kappale: kappale }
    }
}

impl Perusmaailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Perusmaailma {
            kappaleet: Vec::new(),
            fysiikka_kappaleet: Vec::new(),
            pelihahmo: None,
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

    /// Lisää annetun fysiikkakappaleen maailmaan
    /// # Arguments
    /// * `kappale` - Lisättävä fysiikkakappale
    pub fn lisaa_fysiikkakappale(&mut self, kappale: Fysiikkakappale) {
        self.fysiikka_kappaleet.push(kappale);
    }

    /// Lisää annetun pelihahmon maailmaan, jos maailmassa ei jo ole pelihahmoa
    pub fn lisaa_pelihahmo(&mut self, pelihahmo: Kappale) {
        if self.pelihahmo.is_none() {
            let pelikappale = self.lisaa_kappale(pelihahmo);
            self.pelihahmo = Some(Pelihahmo::new(pelikappale));
            return;
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
}

impl PiirrettavaMaailma for Perusmaailma {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat(&self, _sijainti: Vektori) -> &[RcKappale] {
        &self.kappaleet
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

/// Ennalta määrätty muoto kuten neliä tai ympyrä
#[derive(Copy, Clone)]
pub enum Muoto {
    /// Tarkkaan ottaen suorakaide, jolla on leveys ja korkeus
    Nelio(f32, f32),
    /// Ympyrä, jolla on säde
    Ympyra(f32),
}

/// Kappale, jolla on muoto ja sijainti
pub struct Kappale {
    /// Kappaleen muoto
    pub muoto: Muoto,
    /// Kappaleen sijainti
    pub sijainti: Vektori<f32>,
}

impl Kappale {
    /// Luo uuden kappaleen
    /// # Arguments
    /// * `muoto` - Kappaleen muoto
    /// * `x` - Kappaleen keskipisteen sijainnin x-koordinaatti
    /// * `y` - Kappaleen keskipisteen sijainnin y-koordinaatti
    pub fn new(muoto: Muoto, x: f32, y: f32) -> Self {
        match muoto {
            Muoto::Nelio(xl, yl) => Kappale {
                muoto: muoto,
                sijainti: Vektori::new(x - xl / 2.0, y - yl / 2.0),
            },
            Muoto::Ympyra(r) => Kappale {
                muoto: muoto,
                sijainti: Vektori::new(x - r, y - r),
            },
        }
    }
}
