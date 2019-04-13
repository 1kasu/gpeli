use std::cell::RefCell;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

use super::fysiikka::Fysiikkakappale;

type RcKappale = Rc<RefCell<Kappale>>;

/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
#[derive(Default)]
pub struct Maailma {
    /// Pelimaailman sisältämät kappaleet
    kappaleet: Vec<RcKappale>,
    /// Maailmassa olevat fysiikkakappaleet
    fysiikka_kappaleet: Vec<Fysiikkakappale>,
    /// Onko pelihahmo luotu jo
    pelihahmo: bool,
}

impl Maailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Maailma {
            kappaleet: Vec::new(),
            fysiikka_kappaleet: Vec::new(),
            pelihahmo: false,
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

    /// Lisää annetun pelihahmon maailmaan
    pub fn lisaa_pelihahmo(&mut self, pelihahmo: Kappale) {
        if !self.pelihahmo {
            self.kappaleet.insert(0, Rc::new(RefCell::new(pelihahmo)));
            self.pelihahmo = true;
        }
    }

    /// Antaa pelihahmon, jos sellainen on luotu
    pub fn anna_pelihahmo_mut(&mut self) -> Option<&mut RcKappale> {
        if self.pelihahmo {
            Some(&mut self.kappaleet[0])
        } else {
            None
        }
    }

    /// Antaa pelihahmon, jos sellainen on luotu
    pub fn anna_pelihahmo(&self) -> Option<&RcKappale> {
        if self.pelihahmo {
            Some(&self.kappaleet[0])
        } else {
            None
        }
    }

    /// Onko maailmassa pelihahmo olemassa
    pub fn onko_pelihahmo(&self) -> bool {
        self.pelihahmo
    }

    /// Antaa piirrettävät kappaleet
    pub fn piirrettavat(&self, _sijainti: Vektori) -> &[RcKappale] {
        &self.kappaleet
    }

    /// Antaa fysiikkalliset kappaleet
    pub fn fysiikalliset(&mut self) -> &mut [Fysiikkakappale] {
        &mut self.fysiikka_kappaleet
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
