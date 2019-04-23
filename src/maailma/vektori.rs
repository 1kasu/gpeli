//! Sisältää 2d-vektoriin liittyvät toiminnot (ja vektorin itsensä)
use std::ops::{Add, Div, Mul, Sub};

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
