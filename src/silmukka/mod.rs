extern crate sdl2;

pub mod perussilmukka;

/// Pelin pääsilmukka, joka huolehtii pelin toiminnasta
pub trait Paasilmukka {
    /// Käynnistää alustetun pääsilmukan
    fn kaynnista_silmukka(&mut self) -> Result<(), String>;
}
