//! Pelimaailman esittämisestä vastaava komponentti.
//! Peli voidaan esittää esimerkiksi piirtämällä näytölle kuva tai
//! lähettämällä pelimaailman tila verkon yli asiakkaalle.
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use crate::maailma::Maailma;
use crate::maailma::Muoto;
use crate::maailma::Sijainti;

/// Huolehtii pelimaailman esittämisestä käyttäjälle.
pub trait Piirtaja {
    /// Esittää pelitilan käyttäjälle jollain tavalla.
    /// # Arguments
    /// * `maailma` - Esitettävä pelimaailma
    fn piirra_maailma(&mut self, maailma: &Maailma) -> Result<(), String>;
    /// Asettaa kameran sijainnin
    /// # Arguments
    /// * `kameran_sijainti` - Piirtavan kameran sijainti
    fn aseta_kameran_sijainti(&mut self, kameran_sijainti: Sijainti) -> Result<(), String>;
    /// Asettaa kameran zoomin
    /// # Arguments
    /// * `kameran_zoomi` - Kuinka paljon kamera zoomaa kuvaa. Suhteellinen luku, jolloin 1.0 on ei-zoomia. Suurempi luku zoomaa.
    fn aseta_kameran_zoomi(&mut self, kameran_zoomi: f32);
    /// Asettaa suhteellisen etaisyyn, jonka verran kamera voi jäädä jälkeen seurattavasta kohteesta.
    /// # Arguments
    /// * `etaisyys` - Kuinka paljon kamera voi jäädä jälkeen seurattavasta. Suhteellinen arvo väliltä 0-1. Sisältää x ja y koordinaatin erikseen.
    fn aseta_kameran_seurauksen_etaisyys(&mut self, etaisyys: (f32, f32)) -> Result<(), String>;
}

/// Peruspiirtäjä, joka piirtää pelin tilan näytölle
pub struct Peruspiirtaja {
    /// Canvas, jolle pelin tila piirretään
    canvas: Canvas<sdl2::video::Window>,
    /// Kamera, jonka näkökulmasta pelimaailma esitetään
    kamera: Kamera,
}

/// Kamera, joka rajaa mikä alue esitetään pelimaailmasta.
struct Kamera {
    /// Kameran sijainti pelimaailmassa
    sijainti: Sijainti,
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
    pub fn new(sijainti: Sijainti, zoomin_kerroin: f32) -> Self {
        Kamera {
            sijainti: sijainti,
            zoomin_kerroin: zoomin_kerroin,
            etaisyys_seurattavasta: (0.0, 0.0),
        }
    }
}

impl Peruspiirtaja {
    /// Luo uuden peruspiirtäjän
    /// # Arguments
    /// * `canvas` - Canvas, jolle kuva piirretään
    pub fn new(canvas: Canvas<sdl2::video::Window>) -> Result<Self, String> {
        Ok(Peruspiirtaja {
            kamera: Kamera::new(Peruspiirtaja::canvaksen_keskipiste(&canvas)?, 1.0),
            canvas: canvas,
        })
    }

    /// Laskee kameran aiheuttaman sijainnin muutoksen ja palauttaa sen
    fn kameran_aiheuttama_muutos(&self) -> Result<(Sijainti), String> {
        let keskipiste = self.keskipiste()?;
        let muutos = keskipiste - self.kamera.sijainti;
        Ok(muutos)
    }

    /// Antaa piirtoalueen keskipisteen
    /// # Arguments
    /// * `canvas` - Piirtoalue, jonka keskipiste lasketaan
    fn canvaksen_keskipiste(canvas: &Canvas<sdl2::video::Window>) -> Result<Sijainti, String> {
        let koko = canvas.output_size()?;
        Ok(Sijainti::new(koko.0 as f32 / 2.0, koko.1 as f32 / 2.0))
    }

    /// Antaa piirtoalueen keskipisteen
    fn keskipiste(&self) -> Result<Sijainti, String> {
        Peruspiirtaja::canvaksen_keskipiste(&self.canvas)
    }
}

impl Piirtaja for Peruspiirtaja {
    /// Piirtää kuvan pelimaailman tilasta.
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka pohjalta kuva piirretään
    fn piirra_maailma(&mut self, maailma: &Maailma) -> Result<(), String> {
        // Lasketaan kameran aiheuttama muutos
        let muutos = self.kameran_aiheuttama_muutos()?;

        self.canvas.set_draw_color(Color::RGB(10, 100, 10));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(200, 100, 10));

        for kappale in &maailma.kappaleet {
            match kappale.muoto {
                Muoto::Nelio(leveys, korkeus) => {
                    self.canvas.fill_rect(Some(Rect::new(
                        (kappale.sijainti.x * self.kamera.zoomin_kerroin + muutos.x) as i32,
                        (kappale.sijainti.y * self.kamera.zoomin_kerroin + muutos.y) as i32,
                        (leveys * self.kamera.zoomin_kerroin) as u32,
                        (korkeus * self.kamera.zoomin_kerroin) as u32,
                    )))?;
                }
                Muoto::Ympyra(_) => (),
            }
        }
        self.canvas.present();

        Ok(())
    }

    /// Asettaa kameran sijainnin eli missä kohtaa pelimaailmaa kuvan keskipisteen tulisi olla.
    /// # Arguments
    /// * `sijainti` - Kameran sijainti
    fn aseta_kameran_sijainti(&mut self, sijainti: Sijainti) -> Result<(), String> {
        let zoomattu_sijainti = sijainti.kerro(self.kamera.zoomin_kerroin);

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