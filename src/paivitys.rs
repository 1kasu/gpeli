use std::time::Duration;

use crate::maailma::*;
use crate::syotteet::*;

/// Huolehtii pelin toiminnasta esim. pelimaailman alustuksesta ja pelin päivityksestä
pub trait Paivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        events: &sdl2::EventPump,
    );

    /// Päivittää annetun pelimaailman tilan annetuilla syötteillä ja päivitysajalla
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        paivitys_aika: &Paivitysaika,
    );
}

/// Sisältää tiedon kuinka paljon peliä päivitetään ja kuinka paljon aikaa on kulunut pelin alusta
pub struct Paivitysaika<'a>{
    /// Kuinka paljon peliä päivitetään
    pub paivitysaika: &'a Duration,
    /// Kuinka paljona aikaa on kulunut pelin alusta
    pub kokonais_pelin_aika: &'a Duration
}

impl<'a> Paivitysaika<'a>{
    /// Luo uuden päivitysaika otuksen
    /// # Arguments
    /// * `paivitysaika` - Kuinka paljon päivitetään
    /// * `kokonais_pelin_aika` - Kokonaisaika pelin alusta
    pub fn new(paivitysaika: &'a Duration, kokonais_pelin_aika: &'a Duration) -> Paivitysaika<'a>{
        Paivitysaika{paivitysaika: paivitysaika, kokonais_pelin_aika: kokonais_pelin_aika}
    }
}

/// Päivitys, joka sisältää useampia eri päivityksiä
pub struct YhdistettyPaivitys<'a> {
    /// Lista päivityksistä
    pub paivitykset: Vec<&'a mut Paivitys>,
}

impl<'a> YhdistettyPaivitys<'a> {
    pub fn new(lista: Vec<&'a mut Paivitys>) -> YhdistettyPaivitys<'a> {
        YhdistettyPaivitys { paivitykset: lista }
    }
}

impl<'a> Paivitys for YhdistettyPaivitys<'a> {
    /// Suorittaa alustuksen kaikille yhdistetyille paivityksille järjestyksessä, jossa vasen on ensin.
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        events: &sdl2::EventPump,
    ) {
        for paivitys in &mut self.paivitykset {
            paivitys.alusta(maailma, syotteet, events);
        }
    }

    /// Suorittaa päivityksen kaikille yhdistetyille päivityksille järjestyksessä, jossa vasen on ensin
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        paivitys_aika: &Paivitysaika,
    ) {
        for paivitys in &mut self.paivitykset {
            paivitys.paivita(maailma, syotteet, paivitys_aika);
        }
    }
}
