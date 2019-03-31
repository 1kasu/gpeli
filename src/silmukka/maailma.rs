/// Sisältää tiedon pelimaailman tilasta eli kaikkien kappaleiden tiedot
pub struct Maailma {
    /// Pelimaailman sisältämät kappaleet
    pub kappaleet: Vec<Kappale>
}

impl Maailma {
    /// Luo uuden tyhjän maailman
    pub fn new() -> Self {
        Maailma{kappaleet: Vec::new()}
    }
    
    /// Lisää annetun kappaleen maailmaan
    pub fn lisaa_kappale(&mut self, kappale: Kappale){
        self.kappaleet.push(kappale);
    }
}

/// Sijainti 2d maailmassa
pub struct Sijainti<T> {
    /// x-koordinaatti
    pub x: T,
    /// y-koordinaatti
    pub y: T
}

impl Sijainti<f32> {
    /// Siirtää sijaintia annetun verran
    pub fn liiku(&mut self, x: f32, y: f32){
        self.x += x;
        self.y += y;
    }
}

/// Ennalta määrätty muoto kuten neliä tai ympyrä
pub enum Muoto{
    /// Tarkkaan ottaen suorakaide, jolla on leveys ja korkeus
    Nelio(u32, u32),
    /// Ympyrä, jolla on säde
    Ympyra(u32)
}


/// Kappale, jolla on muoto ja sijainti
pub struct Kappale{
    /// Kappaleen muoto
    pub muoto: Muoto,
    /// Kappaleen sijainti
    pub sijainti: Sijainti<f32>
}

impl Kappale {
    pub fn new(muoto: Muoto, x: f32, y: f32) -> Self{
        Kappale{muoto: muoto, sijainti: Sijainti{x,y}}
    }
}