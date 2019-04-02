use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

use crate::maailma::Maailma;
use crate::maailma::Muoto;

/// Huolehtii pelimaailman esittämisestä käyttäjälle.
pub trait Piirtaja {
    /// Esittää pelitilan käyttäjälle jollain tavalla.
    /// # Arguments
    /// 
    /// * `maailma - Esitettävä pelimaailma
    fn piirra_maailma(&mut self, maailma: &Maailma) -> Result<(), String>;
}

/// Peruspiirtäjä, joka piirtää pelin tilan näytölle
pub struct Peruspiirtaja {
    /// Canvas, jolle pelin tila piirretään
    canvas: Canvas<sdl2::video::Window>
}

impl Peruspiirtaja{
    /// Luo uuden peruspiirtäjän
    /// # Arguments
    /// * `canvas - Canvas, jolle kuva piirretään
    pub fn new(canvas:  Canvas<sdl2::video::Window>) -> Self{
        Peruspiirtaja{canvas: canvas}
    }
}

impl Piirtaja for Peruspiirtaja {
    /// Piirtää kuvan pelimaailman tilasta.
    /// # Arguments
    /// * `maailma - Pelimaailma, jonka pohjalta kuva piirretään
    fn piirra_maailma(
        &mut self,
        maailma: &Maailma,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(10, 100, 10));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(200, 100, 10));

        for kappale in &maailma.kappaleet {
            match kappale.muoto {
                Muoto::Nelio(leveys, korkeus) => {
                    self.canvas.fill_rect(Some(Rect::new(
                        kappale.sijainti.x as i32,
                        kappale.sijainti.y as i32,
                        leveys,
                        korkeus,
                    )))?;
                }
                Muoto::Ympyra(_) => (),
            }
        }
        self.canvas.present();

        Ok(())
    }
}
