extern crate sdl2;

pub mod perussilmukka;
pub mod saannollinensilmukka;

/// Pelin pääsilmukka, joka huolehtii pelin toiminnasta
pub trait Paasilmukka : std::fmt::Display{
    /// Käynnistää alustetun pääsilmukan
    fn kaynnista_silmukka(&mut self) -> Result<(), String>;
}
