//! Sisältää kappaleen ja siihen sisältyvät osat
use super::Vektori;

/// Ennalta määrätty muoto kuten neliä tai ympyrä
#[derive(Copy, Clone)]
pub enum Muoto {
    /// Tarkkaan ottaen suorakaide, jolla on leveys ja korkeus
    Nelio(f32, f32),
    /// Ympyrä, jolla on säde
    Ympyra(f32),
}

impl Muoto {
    /// Antaa keskipisteen sijainnin suhteessa vasempaan yläkulmaan
    pub fn keskipiste(&self) -> Vektori {
        match &self {
            Muoto::Nelio(leveys, korkeus) => Vektori::new(leveys / 2.0, korkeus / 2.0),
            Muoto::Ympyra(sade) => Vektori::new(*sade, *sade),
        }
    }

    /// Antaa muodon maksimi leveyden ja korkeuden.
    /// Esim. ympyrällä halkaisijat
    pub fn koko(&self) -> (f32, f32) {
        match &self {
            Muoto::Nelio(leveys, korkeus) => (*leveys, *korkeus),
            Muoto::Ympyra(sade) => (sade * 2.0, sade * 2.0),
        }
    }
}

/// Kertoo minkälainen kappale on kyseessä.
#[derive(PartialEq, Copy, Clone)]
pub enum Tagi {
    Vihollinen,
    Seina,
    Ammus,
    Pelaaja,
    Partikkeli,
}

/// Kappale, jolla on muoto ja sijainti
pub struct Kappale {
    /// Kappaleen muoto
    pub muoto: Muoto,
    /// Kappaleen kulman sijainti
    sijainti: Vektori<f32>,
    /// Minkälainen kappale on kyseessä
    pub tagi: Tagi,
}

impl Kappale {
    /// Luo uuden kappaleen käyttäen keskipistettä
    /// # Arguments
    /// * `muoto` - Kappaleen muoto
    /// * `x` - Kappaleen keskipisteen sijainnin x-koordinaatti
    /// * `y` - Kappaleen keskipisteen sijainnin y-koordinaatti
    pub fn new_keskipisteella(muoto: Muoto, x: f32, y: f32, tagi: Tagi) -> Self {
        Kappale {
            muoto: muoto,
            sijainti: Vektori::new(x, y) - muoto.keskipiste(),
            tagi: tagi,
        }
    }

    /// Luo uuden kappaleen käyttäen vasenta yläkulmaa
    /// # Arguments
    /// * `muoto` - Kappaleen muoto
    /// * `x` - Kappaleen kulman sijainnin x-koordinaatti
    /// * `y` - Kappaleen kulman sijainnin y-koordinaatti
    pub fn new_kulmalla(muoto: Muoto, x: f32, y: f32, tagi: Tagi) -> Self {
        Kappale {
            muoto: muoto,
            sijainti: Vektori::new(x, y),
            tagi: tagi,
        }
    }

    /// Antaa kappaleen vasemman yläkulman(?) sijainnin
    pub fn kulman_sijainti(&self) -> Vektori {
        self.sijainti
    }

    /// Antaa kappaleen keskipisteen sijainnin
    pub fn keskipisteen_sijainti(&self) -> Vektori {
        self.sijainti + self.muoto.keskipiste()
    }

    /// Asettaa kappaleen vasemman yläkulman sijainnin
    /// # Arguments
    /// * `uusi_sijainti` - Uusi kulman sijainti
    pub fn aseta_kulman_sijainti(&mut self, uusi_sijainti: Vektori) {
        self.sijainti = uusi_sijainti;
    }

    /// Asettaa kappaleen keskipisteen sijainnin
    /// # Arguments
    /// * `uusi_sijainti` - Uusi keskipisteen sijainti
    pub fn aseta_keskipisteen_sijainti(&mut self, uusi_sijainti: Vektori) {
        self.sijainti = uusi_sijainti - self.muoto.keskipiste();
    }
}
