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
    /// * `kamera` - Kameran sijainti
    fn aseta_kameran_sijainti(&mut self, kamera: Sijainti);
}

/// Peruspiirtäjä, joka piirtää pelin tilan näytölle
pub struct Peruspiirtaja {
    /// Canvas, jolle pelin tila piirretään
    canvas: Canvas<sdl2::video::Window>,
    /// Kameran sijainti
    kamera: Sijainti,
}

impl Peruspiirtaja {
    /// Luo uuden peruspiirtäjän
    /// # Arguments
    /// * `canvas` - Canvas, jolle kuva piirretään
    pub fn new(canvas: Canvas<sdl2::video::Window>) -> Self {
        // TODO: Aseta kameran sijainti oletuksena keskelle
        Peruspiirtaja {
            canvas: canvas,
            kamera: Sijainti::new(0.0, 0.0),
        }
    }

    /// Laskee kameran aiheuttaman sijainnin muutoksen ja palauttaa sen
    fn kameran_aiheuttama_muutos(&self) -> Result<((f32,f32)), String> {
        let koko = self.canvas.output_size()?;
        let muutos = (
            koko.0 as f32 / 2.0 - self.kamera.x,
            koko.1 as f32 / 2.0 - self.kamera.y,
        );
        Ok(muutos)
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
                        kappale.sijainti.x as i32 + muutos.0 as i32,
                        kappale.sijainti.y as i32 + muutos.1 as i32,
                        leveys as u32,
                        korkeus as u32,
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
    /// * `kamera` - Kameran sijainti
    fn aseta_kameran_sijainti(&mut self, kamera: Sijainti) {
        self.kamera = kamera;
    }
}
