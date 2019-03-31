extern crate sdl2;


use sdl2::render::Canvas;

pub mod perussilmukka;
pub mod maailma;

/// Pelin pääsilmukka, joka huolehtii pelin toiminnasta
pub trait Paasilmukka {
    /// Luo uuden pääsilmukan
    ///
    /// # Arguments
    ///
    /// * `canvas - Canvas, jolle piirretään
    /// * `events - Tapahtumat
    /// * `context - SDL context, josta voidaan kysyä esim. ajastin.
    fn new(
        canvas: Canvas<sdl2::video::Window>,
        events: sdl2::EventPump,
        context: sdl2::Sdl,
    ) -> Self;
    /// Käynnistää alustetun pääsilmukan
    fn kaynnista_silmukka(&mut self) -> Result<(), String>;
}


