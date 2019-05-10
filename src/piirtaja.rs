//! Pelimaailman esittämisestä vastaava komponentti.
//! Peli voidaan esittää esimerkiksi piirtämällä näytölle kuva tai
//! lähettämällä pelimaailman tila verkon yli asiakkaalle.
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::maailma::kappale::{Kappale, Muoto};
use crate::maailma::vektori::Vektori;
use crate::maailma::Lisaosa;

/// Huolehtii pelimaailman esittämisestä käyttäjälle.
pub trait Piirtovalmius {
    /// Esittää valmistellun kuvan käyttäjälle
    fn esita_kuva(&mut self);
    /// Pyyhkii vanhan kuvan pois
    fn puhdista_kuva(&mut self);
    /// Asettaa kameran sijainnin
    /// # Arguments
    /// * `kameran_sijainti` - Piirtavan kameran sijainti
    fn aseta_kameran_sijainti(&mut self, kameran_sijainti: Vektori) -> Result<(), String>;
    /// Asettaa kameran zoomin
    /// # Arguments
    /// * `kameran_zoomi` - Kuinka paljon kamera zoomaa kuvaa. Suhteellinen luku, jolloin 1.0 on ei-zoomia. Suurempi luku zoomaa.
    fn aseta_kameran_zoomi(&mut self, kameran_zoomi: f32);
    /// Asettaa suhteellisen etaisyyn, jonka verran kamera voi jäädä jälkeen seurattavasta kohteesta.
    /// # Arguments
    /// * `etaisyys` - Kuinka paljon kamera voi jäädä jälkeen seurattavasta. Suhteellinen arvo väliltä 0-1. Sisältää x ja y koordinaatin erikseen.
    fn aseta_kameran_seurauksen_etaisyys(&mut self, etaisyys: (f32, f32)) -> Result<(), String>;
}

pub trait ValiaikaistenPiirtaja: Piirtovalmius {
    /// Esittää pelitilan käyttäjälle jollain tavalla.
    /// # Arguments
    /// * `piirrettavat` - Lista piirrettävistä kappaleista
    fn piirra_kappaleista(
        &mut self,
        piirrettavat: &[ValiaikainenPiirrettavaKappale],
    ) -> Result<(), String>;
}

pub trait MaailmanPiirtaja: Piirtovalmius {
    /// Esittää pelitilan käyttäjälle jollain tavalla.
    /// # Arguments
    /// * `maailma` - Esitettävä pelimaailma
    fn piirra_maailma(&mut self, maailma: &PiirrettavaMaailma) -> Result<(), String>;
}

pub trait PiirrettavaMaailma {
    /// Piirrettävät kappaleet maailmassa
    /// # Arguments
    /// * `sijainti` - Ilmoittaa mistä päin maailmaa halutaan piirrettävät kappaleet
    fn piirrettavat<'a>(
        &'a self,
        sijainti: Vektori,
    ) -> Box<Iterator<Item = &'a PiirrettavaKappale> + 'a>;

    /// Antaa kameran sijainnin pelimaailmassa, jos maailma haluaa ehdottaa jotakin
    fn anna_kameran_sijainti(&self) -> Option<Vektori>;
}

type RcKappale = Rc<RefCell<Kappale>>;

/// Kappale, joka voidaan piirtää
pub struct PiirrettavaKappale {
    /// Piirrettävä kappale
    kappale: RcKappale,
    /// Millä tavalla piirtäminen tehdään
    piirtotapa: Piirtotapa,
}

/// Kappale, joka piirretään interpoloimalla kahden muistin välillä
pub struct ValiaikainenPiirrettavaKappale {
    /// Piirrettävä kappale
    kappale: Kappale,
    /// Millä tavalla piirtäminen tehdään
    piirtotapa: Piirtotapa,
}

impl PiirrettavaKappale {
    /// Luo uuden piirrettävän kappaleen
    /// # Arguments
    /// * `kappale` - Piirrettävä kappale
    /// * `piirtotapa` - Tapa, jolla kappale piirretään
    pub fn new(kappale: RcKappale, piirtotapa: Piirtotapa) -> Self {
        PiirrettavaKappale {
            kappale: kappale,
            piirtotapa: piirtotapa,
        }
    }

    pub fn anna_piirtotapa(&self) -> &Piirtotapa {
        &self.piirtotapa
    }
}

impl ValiaikainenPiirrettavaKappale {
    /// Luo uuden piirrettävän kappaleen
    /// # Arguments
    /// * `kappale` - Piirrettävä kappale
    /// * `piirtotapa` - Tapa, jolla kappale piirretään
    pub fn new(kappale: Kappale, piirtotapa: Piirtotapa) -> Self {
        ValiaikainenPiirrettavaKappale {
            kappale: kappale,
            piirtotapa: piirtotapa,
        }
    }

    pub fn anna_piirtotapa(&self) -> &Piirtotapa {
        &self.piirtotapa
    }
}

/// Piirtämisessä käytettävä tapa
#[derive(Clone)]
pub enum Piirtotapa {
    /// Piirretään yksivärisenä
    Yksivarinen { vari: Color },
    /// Piirretään kuvan avulla
    Kuvallinen { kuvan_nimi: String },
}

impl Lisaosa for PiirrettavaKappale {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
    }
}

/// Kohde, joka on piirrettävissä canvakselle
pub trait Piirrettava {
    /// Piirtää kohteen canvakselle käyttämällä tarvittavia kameran muunnoksia
    /// # Arguments
    /// * `canvas` - Canvas, jolle piirretään
    /// * `kameran_aiheuttama_muunnos` - Kameran sijainnista johtuva muunnos
    /// * `kameran_zoomaus` - Kameran zoomauksesta johtuva muunnos
    /// * `tekstuurit` - Käytössä olevat tekstuurit
    fn piirra(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        kameran_aiheuttama_muutos: Vektori,
        kameran_zoomaus: f32,
        tekstuurit: &HashMap<String, Texture>,
    ) -> Result<(), String>;
}

impl Kappale {
    /// Piirtää kappaleen canvakselle käyttämällä tarvittavia kameran muunnoksia
    /// # Arguments
    /// * `canvas` - Canvas, jolle piirretään
    /// * `kameran_aiheuttama_muunnos` - Kameran sijainnista johtuva muunnos
    /// * `kameran_zoomaus` - Kameran zoomauksesta johtuva muunnos
    fn piirra(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        kameran_aiheuttama_muutos: Vektori,
        kameran_zoomaus: f32,
    ) -> Result<(), String> {
        let sijainti = self.kulman_sijainti() * kameran_zoomaus + kameran_aiheuttama_muutos;
        match self.muoto {
            Muoto::Nelio(leveys, korkeus) => {
                canvas.fill_rect(Some(Rect::new(
                    sijainti.x as i32,
                    sijainti.y as i32,
                    (leveys * kameran_zoomaus) as u32,
                    (korkeus * kameran_zoomaus) as u32,
                )))?;
            }
            Muoto::Ympyra(sade) => {
                canvas.fill_rect(Some(Rect::new(
                    sijainti.x as i32,
                    sijainti.y as i32,
                    (sade * 2.0 * kameran_zoomaus) as u32,
                    (sade * 2.0 * kameran_zoomaus) as u32,
                )))?;
            }
        }

        Ok(())
    }

    /// Piirtää käyttämällä annettua tekstuuria
    /// # Arguments
    /// * `canvas` - Canvas, jolle piirretään
    /// * `kameran_aiheuttama_muunnos` - Kameran sijainnista johtuva muunnos
    /// * `kameran_zoomaus` - Kameran zoomauksesta johtuva muunnos
    /// * `tekstuuri` - Piirrettäessä käytettävä tekstuuri
    fn piirra_kuvalla(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        kameran_aiheuttama_muutos: Vektori,
        kameran_zoomaus: f32,
        tekstuuri: &Texture,
    ) -> Result<(), String> {
        let sijainti = self.kulman_sijainti() * kameran_zoomaus + kameran_aiheuttama_muutos;
        match self.muoto {
            Muoto::Nelio(leveys, korkeus) => {
                canvas.copy(
                    tekstuuri,
                    None,
                    Some(Rect::new(
                        sijainti.x as i32,
                        sijainti.y as i32,
                        (leveys * kameran_zoomaus) as u32,
                        (korkeus * kameran_zoomaus) as u32,
                    )),
                )?;
            }
            Muoto::Ympyra(sade) => {
                canvas.copy(
                    tekstuuri,
                    None,
                    Some(Rect::new(
                        sijainti.x as i32,
                        sijainti.y as i32,
                        (sade * 2.0 * kameran_zoomaus) as u32,
                        (sade * 2.0 * kameran_zoomaus) as u32,
                    )),
                )?;
            }
        }

        Ok(())
    }
}

impl Piirrettava for PiirrettavaKappale {
    /// Piirtää kappaleen canvakselle käyttämällä tarvittavia kameran muunnoksia
    /// # Arguments
    /// * `canvas` - Canvas, jolle piirretään
    /// * `kameran_aiheuttama_muunnos` - Kameran sijainnista johtuva muunnos
    /// * `kameran_zoomaus` - Kameran zoomauksesta johtuva muunnos
    /// * `tesktuurit` - Käytössä olevat tekstuurit
    fn piirra(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        kameran_aiheuttama_muutos: Vektori,
        kameran_zoomaus: f32,
        tekstuurit: &HashMap<String, Texture>,
    ) -> Result<(), String> {
        match &self.piirtotapa {
            Piirtotapa::Yksivarinen { vari: v } => {
                canvas.set_draw_color(v.rgba());
                self.kappale
                    .borrow()
                    .piirra(canvas, kameran_aiheuttama_muutos, kameran_zoomaus)?;
            }
            Piirtotapa::Kuvallinen { kuvan_nimi: kuva } => {
                if let Some(kuva) = tekstuurit.get(kuva) {
                    self.kappale.borrow().piirra_kuvalla(
                        canvas,
                        kameran_aiheuttama_muutos,
                        kameran_zoomaus,
                        kuva,
                    )?;
                } else {
                    // Kuva ei löydy, joten piirretään punaisella päälle
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    self.kappale.borrow().piirra(
                        canvas,
                        kameran_aiheuttama_muutos,
                        kameran_zoomaus,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Piirrettava for ValiaikainenPiirrettavaKappale {
    /// Piirtää kappaleen canvakselle käyttämällä tarvittavia kameran muunnoksia
    /// # Arguments
    /// * `canvas` - Canvas, jolle piirretään
    /// * `kameran_aiheuttama_muunnos` - Kameran sijainnista johtuva muunnos
    /// * `kameran_zoomaus` - Kameran zoomauksesta johtuva muunnos
    /// * `tesktuurit` - Käytössä olevat tekstuurit
    fn piirra(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        kameran_aiheuttama_muutos: Vektori,
        kameran_zoomaus: f32,
        tekstuurit: &HashMap<String, Texture>,
    ) -> Result<(), String> {
        match &self.piirtotapa {
            Piirtotapa::Yksivarinen { vari: v } => {
                canvas.set_draw_color(v.rgba());
                self.kappale
                    .piirra(canvas, kameran_aiheuttama_muutos, kameran_zoomaus)?;
            }
            Piirtotapa::Kuvallinen { kuvan_nimi: kuva } => {
                if let Some(kuva) = tekstuurit.get(kuva) {
                    self.kappale.piirra_kuvalla(
                        canvas,
                        kameran_aiheuttama_muutos,
                        kameran_zoomaus,
                        kuva,
                    )?;
                } else {
                    // Kuva ei löydy, joten piirretään punaisella päälle
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    self.kappale
                        .piirra(canvas, kameran_aiheuttama_muutos, kameran_zoomaus)?;
                }
            }
        }
        Ok(())
    }
}

/// Peruspiirtäjä, joka piirtää pelin tilan näytölle
pub struct Peruspiirtaja<'a> {
    /// Canvas, jolle pelin tila piirretään
    canvas: Canvas<sdl2::video::Window>,
    /// Kamera, jonka näkökulmasta pelimaailma esitetään
    kamera: Kamera,
    /// Käytössä olevat tekstuurit
    tekstuurit: HashMap<String, Texture<'a>>,
}

/// Kamera, joka rajaa mikä alue esitetään pelimaailmasta.
struct Kamera {
    /// Kameran sijainti pelimaailmassa
    sijainti: Vektori,
    /// Kerroin, jolla zoomataan piirrettäviä kohteita.
    zoomin_kerroin: f32,
    /// Suhteellinen etäisyys kuinka paljon kamera voi jäädä jälkeen seurattavasta kohteesta.
    etaisyys_seurattavasta: (f32, f32),
}

impl Kamera {
    /// Luo uuden kameran
    /// # Arguments
    /// * `sijainti` - Kameran sijainti pelimaailmassa
    /// * `zoomin_kerroin` - Kuinka paljon kamera zoomaa kuvaa. Suhteellinen luku, jolloin 1.0 on ei-zoomia. Suurempi luku zoomaa.
    pub fn new(sijainti: Vektori, zoomin_kerroin: f32) -> Self {
        Kamera {
            sijainti: sijainti,
            zoomin_kerroin: zoomin_kerroin,
            etaisyys_seurattavasta: (0.0, 0.0),
        }
    }
}

impl<'a> Peruspiirtaja<'a> {
    /// Luo uuden peruspiirtäjän
    /// # Arguments
    /// * `canvas` - Canvas, jolle kuva piirretään
    pub fn new(canvas: Canvas<sdl2::video::Window>) -> Result<Self, String> {
        Ok(Peruspiirtaja {
            kamera: Kamera::new(Peruspiirtaja::canvaksen_keskipiste(&canvas)?, 1.0),
            canvas: canvas,
            tekstuurit: HashMap::new(),
        })
    }

    /// Laskee kameran aiheuttaman sijainnin muutoksen ja palauttaa sen
    fn kameran_aiheuttama_muutos(&self) -> Result<(Vektori), String> {
        let keskipiste = self.keskipiste()?;
        let muutos = keskipiste - self.kamera.sijainti;
        Ok(muutos)
    }

    /// Antaa piirtoalueen keskipisteen
    /// # Arguments
    /// * `canvas` - Piirtoalue, jonka keskipiste lasketaan
    fn canvaksen_keskipiste(canvas: &Canvas<sdl2::video::Window>) -> Result<Vektori, String> {
        let koko = canvas.output_size()?;
        Ok(Vektori::new(koko.0 as f32 / 2.0, koko.1 as f32 / 2.0))
    }

    /// Antaa piirtoalueen keskipisteen
    fn keskipiste(&self) -> Result<Vektori, String> {
        Peruspiirtaja::canvaksen_keskipiste(&self.canvas)
    }

    /// Lisää annetun tekstuurin käytettäväksi
    /// # Arguments
    /// * `tekstuuri` - Lisättävä tekstuuri
    /// * `nimi` - Tekstuurin nimi
    pub fn lisaa_tekstuuri(&mut self, tekstuuri: Texture<'a>, nimi: String) {
        self.tekstuurit.insert(nimi, tekstuuri);
    }
}

impl<'a> Piirtovalmius for Peruspiirtaja<'a> {
    fn esita_kuva(&mut self) {
        self.canvas.present();
    }

    fn puhdista_kuva(&mut self) {
        self.canvas.set_draw_color(Color::RGB(66, 88, 37));
        self.canvas.clear();
    }

    /// Asettaa kameran sijainnin eli missä kohtaa pelimaailmaa kuvan keskipisteen tulisi olla.
    /// # Arguments
    /// * `sijainti` - Kameran sijainti
    fn aseta_kameran_sijainti(&mut self, sijainti: Vektori) -> Result<(), String> {
        let zoomattu_sijainti = sijainti * self.kamera.zoomin_kerroin;

        self.kamera.sijainti.x = match self.kamera.sijainti.x - zoomattu_sijainti.x {
            x if x < -self.kamera.etaisyys_seurattavasta.0 * self.keskipiste()?.x => {
                zoomattu_sijainti.x - self.kamera.etaisyys_seurattavasta.0 * self.keskipiste()?.x
            }
            x if x > self.kamera.etaisyys_seurattavasta.0 * self.keskipiste()?.x => {
                zoomattu_sijainti.x + self.kamera.etaisyys_seurattavasta.0 * self.keskipiste()?.x
            }
            _ => self.kamera.sijainti.x,
        };
        self.kamera.sijainti.y = match self.kamera.sijainti.y - zoomattu_sijainti.y {
            y if y < -self.kamera.etaisyys_seurattavasta.1 * self.keskipiste()?.y => {
                zoomattu_sijainti.y - self.kamera.etaisyys_seurattavasta.1 * self.keskipiste()?.y
            }
            y if y > self.kamera.etaisyys_seurattavasta.1 * self.keskipiste()?.y => {
                zoomattu_sijainti.y + self.kamera.etaisyys_seurattavasta.1 * self.keskipiste()?.y
            }
            _ => self.kamera.sijainti.y,
        };
        Ok(())
    }

    /// Asettaa kameran zoomin. Jos zoomi pienempi kuin 0.1, niin pakotetaan arvoksi 0.1.
    /// # Arguments
    /// * `kameran_zoomi` - Kuinka paljon kamera zoomaa kuvaa. Suhteellinen luku, jolloin 1.0 on ei-zoomia. Suurempi luku zoomaa.
    fn aseta_kameran_zoomi(&mut self, kameran_zoomi: f32) {
        self.kamera.zoomin_kerroin = match kameran_zoomi {
            x if x <= 0.1 => 0.1,
            x => x,
        }
    }

    /// Asettaa suhteellisen etaisyyn, jonka verran kamera voi jäädä jälkeen seurattavasta kohteesta.
    /// # Arguments
    /// * `etaisyys` - Kuinka paljon kamera voi jäädä jälkeen seurattavasta. Suhteellinen arvo väliltä 0-1. Sisältää x ja y koordinaatin erikseen.
    fn aseta_kameran_seurauksen_etaisyys(&mut self, etaisyys: (f32, f32)) -> Result<(), String> {
        // Rajoitetaan suhteellinen etäisyys välille 0-1
        self.kamera.etaisyys_seurattavasta.0 = match etaisyys.0 {
            x if x <= 0.0 => 0.0,
            x if x >= 1.0 => 1.0,
            x => x,
        };
        self.kamera.etaisyys_seurattavasta.1 = match etaisyys.1 {
            y if y <= 0.0 => 0.0,
            y if y >= 1.0 => 1.0,
            y => y,
        };
        Ok(())
    }
}

impl<'a> MaailmanPiirtaja for Peruspiirtaja<'a> {
    /// Piirtää kuvan pelimaailman tilasta.
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka pohjalta kuva piirretään
    fn piirra_maailma(&mut self, maailma: &PiirrettavaMaailma) -> Result<(), String> {
        if let Some(sijainti) = maailma.anna_kameran_sijainti() {
            self.aseta_kameran_sijainti(sijainti)?;
        }
        // Lasketaan kameran aiheuttama muutos
        let muutos = self.kameran_aiheuttama_muutos()?;

        self.canvas.set_draw_color(Color::RGB(200, 100, 10));

        for piirrettava in maailma.piirrettavat(muutos) {
            piirrettava.piirra(
                &mut self.canvas,
                muutos,
                self.kamera.zoomin_kerroin,
                &self.tekstuurit,
            )?;
        }

        Ok(())
    }
}

impl<'a> ValiaikaistenPiirtaja for Peruspiirtaja<'a> {
    /// Esittää pelitilan käyttäjälle jollain tavalla.
    /// # Arguments
    /// * `piirrettavat` - Lista piirrettävistä kappaleista
    fn piirra_kappaleista(
        &mut self,
        piirrettavat: &[ValiaikainenPiirrettavaKappale],
    ) -> Result<(), String> {
        // Lasketaan kameran aiheuttama muutos
        let muutos = self.kameran_aiheuttama_muutos()?;

        self.canvas.set_draw_color(Color::RGB(200, 100, 10));

        for piirrettava in piirrettavat {
            piirrettava.piirra(
                &mut self.canvas,
                muutos,
                self.kamera.zoomin_kerroin,
                &self.tekstuurit,
            )?;
        }

        Ok(())
    }
}
