/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
#[derive(Default)]
pub struct Maailma {
    /// Pelimaailman sisältämät kappaleet
    pub kappaleet: Vec<Kappale>,
}

impl Maailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Maailma {
            kappaleet: Vec::new(),
        }
    }

    /// Lisää annetun kappaleen maailmaan
    /// # Arguments
    ///
    /// * `kappale` - Lisättävä kappale
    pub fn lisaa_kappale(&mut self, kappale: Kappale) {
        self.kappaleet.push(kappale);
    }
}

/// Sijainti 2d maailmassa. Muodoilla vasemman yläkulman sijainti. Origo on vasemmassa yläkulmassa.
#[derive(Copy, Clone)]
pub struct Sijainti {
    /// x-koordinaatti
    pub x: f32,
    /// y-koordinaatti
    pub y: f32,
}

impl Sijainti {
    /// Siirtää sijaintia annetun verran
    /// # Arguments
    ///
    /// * `x` - x-koordinaatin muutos
    /// * 'y` - y-koordinaatin muutos
    pub fn liiku(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    /// Luo uuden sijainnin
    /// # Arguments
    /// * `x` - sijainnin x-koordinaatti
    /// * `y` - sijainnin y-koordinaatti
    pub fn new(x: f32, y: f32) -> Self {
        Sijainti { x, y }
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
    pub sijainti: Sijainti,
}

impl Kappale {
    /// Luo uuden kappaleen
    /// # Arguments
    ///
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
            }, /*
               _ => Kappale {
                   muoto: muoto,
                   sijainti: Sijainti { x, y },
               },*/
        }
    }
}
