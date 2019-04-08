use std::ops::{Add, Mul, Sub};

/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
#[derive(Default)]
pub struct Maailma {
    /// Pelimaailman sisältämät kappaleet
    kappaleet: Vec<Kappale>,
    /// Onko pelihahmo luotu jo
    pelihahmo: bool,
}

impl Maailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Maailma {
            kappaleet: Vec::new(),
            pelihahmo: false,
        }
    }

    /// Lisää annetun kappaleen maailmaan
    /// # Arguments
    ///
    /// * `kappale` - Lisättävä kappale
    pub fn lisaa_kappale(&mut self, kappale: Kappale) {
        self.kappaleet.push(kappale);
    }

    /// Lisää annetun pelihahmon maailmaan
    pub fn lisaa_pelihahmo(&mut self, pelihahmo: Kappale) {
        if !self.pelihahmo {
            self.kappaleet.insert(0, pelihahmo);
            self.pelihahmo = true;
        }
    }

    /// Antaa pelihahmon, jos sellainen on luotu
    pub fn anna_pelihahmo(&mut self) -> Option<&mut Kappale> {
        if self.pelihahmo {
            Some(&mut self.kappaleet[0])
        } else {
            None
        }
    }

    /// Antaa piirrettävät kappaleet
    pub fn piirrettavat(&self, _sijainti: Sijainti) -> &[Kappale] {
        &self.kappaleet
    }
}

/// Sijainti 2d maailmassa. Muodoilla vasemman yläkulman sijainti. Origo on vasemmassa yläkulmassa.
#[derive(Copy, Clone)]
pub struct Sijainti<T = f32> {
    /// x-koordinaatti
    pub x: T,
    /// y-koordinaatti
    pub y: T,
}

impl<T> Sijainti<T> {
    /// Luo uuden sijainnin
    /// # Arguments
    /// * `x` - sijainnin x-koordinaatti
    /// * `y` - sijainnin y-koordinaatti
    pub fn new(x: T, y: T) -> Self {
        Sijainti { x, y }
    }
}

impl<T: std::ops::AddAssign> Sijainti<T> {
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
impl<T> Mul<T> for Sijainti<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Sijainti<T>;
    fn mul(self, other: T) -> Self::Output {
        Sijainti {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: Add<Output = T>> Add for Sijainti<T> {
    type Output = Sijainti<T>;
    fn add(self, other: Self::Output) -> Self::Output {
        Sijainti {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Sijainti<T> {
    type Output = Sijainti<T>;

    fn sub(self, other: Self::Output) -> Self::Output {
        Sijainti {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Ennalta määrätty muoto kuten neliä tai ympyrä
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
    pub sijainti: Sijainti<f32>,
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
                sijainti: Sijainti::new(x - xl / 2.0, y - yl / 2.0),
            },
            Muoto::Ympyra(r) => Kappale {
                muoto: muoto,
                sijainti: Sijainti::new(x - r, y - r),
            },
        }
    }
}
