use sdl2::keyboard::Scancode;

use super::lisaa_kuvallinen_kappale;
use super::Nopeus;
use crate::fysiikka::{Fysiikallinen, Fysiikkakappale};
use crate::maailma::kappale::{Kappale, Muoto, Tagi::*};
use crate::maailma::{Lisaosa, LisaosienAntaja, Pelihahmollinen, Perusmaailma};
use crate::paivitys::{Paivitys, Paivitysaika};
use crate::syotteet::*;

// Vakioita eri asioille
const OIKEALLE_LIIKKUMINEN: Scancode = Scancode::Right;
const VASEMMALLE_LIIKKUMINEN: Scancode = Scancode::Left;
const ALAS_LIIKKUMINEN: Scancode = Scancode::Down;
const YLOS_LIIKKUMINEN: Scancode = Scancode::Up;
const AMPUMINEN: Scancode = Scancode::Space;
const PELIHAHMON_NOPEUS: f32 = 120.0;
const AMMUKSEN_NOPEUS: f32 = 260.0;
const AMMUKSEN_LEVEYS: f32 = 5.0;

/// Huolehtii pelihahmon päivityksestä
pub struct PelihahmonPaivitys;

impl Paivitys for PelihahmonPaivitys {
    /// Alustaa pelin
    /// # Arguments
    /// * `maailma` - Pelimaailma, joka alustetaan
    /// * `syotteet` - Alustettavat syotteet
    /// * `events` - Sdl:n osa, jolta voidaan kysyä tapahtumia kuten näppäinten painalluksia
    fn alusta(
        &mut self,
        _maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        events: &sdl2::EventPump,
    ) {
        syotteet.lisaa_nappain(events, OIKEALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, VASEMMALLE_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, YLOS_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, ALAS_LIIKKUMINEN);
        syotteet.lisaa_nappain(events, AMPUMINEN);
    }

    /// Päivittää pelihahmon tilan
    /// # Arguments
    /// * `maailma` - Pelimaailma, jonka tila päivitetään
    /// * `syotteet` - Päivityksessä käytettävät syötteet
    /// * `paivitysaika` - Aika, jonka verran pelimaailmaa paivitetaan
    fn paivita(
        &mut self,
        maailma: &mut Perusmaailma,
        syotteet: &mut Syotteet,
        _paivitysaika: &Paivitysaika,
    ) {
        if let Some(pelihahmo) = maailma.anna_pelihahmo_mut() {
            let mut x = 0.0;
            let mut y = 0.0;

            // Liikutetaan pelihahmoa
            if syotteet.nappain_pohjassa(OIKEALLE_LIIKKUMINEN) {
                x += PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(VASEMMALLE_LIIKKUMINEN) {
                x -= PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(YLOS_LIIKKUMINEN) {
                y -= PELIHAHMON_NOPEUS;
            }
            if syotteet.nappain_pohjassa(ALAS_LIIKKUMINEN) {
                y += PELIHAHMON_NOPEUS;
            }

            let hahmon_kappale = pelihahmo.anna_kappale();
            let pelaajan_nopeus = Nopeus::new(x, y);

            if let Some(hahmon_fysiikka) = maailma.anna_fysiikka_mut(&hahmon_kappale) {
                hahmon_fysiikka.aseta_nopeus(pelaajan_nopeus);
            }

            let pelihahmo = maailma.anna_pelihahmo_mut().unwrap();
            // Päivitetään suunta
            pelihahmo.aseta_suunta(pelaajan_nopeus);

            // Pelihahmon ampuminen
            if syotteet.nappain_painettu(AMPUMINEN) {
                // Lasketaan lisättävän ammuksen sijainti
                let pelaajan_keskipiste = hahmon_kappale.borrow().keskipisteen_sijainti();
                let pelaajan_koko = hahmon_kappale.borrow().muoto.koko();
                let ammuksen_suunta = pelihahmo.anna_suunta();

                let ammuksen_muoto = Muoto::Ympyra(AMMUKSEN_LEVEYS);
                let muutos_kerroin = pelaajan_koko.0 / 2.0 + ammuksen_muoto.koko().0 / 2.0 + 10.0;

                let ammuksen_sijainti = pelaajan_keskipiste + ammuksen_suunta * muutos_kerroin;

                // Lisätään ammus pelaajan katsomissuuntaan vähän matkan päähän
                let r_kappale = lisaa_kuvallinen_kappale(
                    maailma,
                    Kappale::new_keskipisteella(
                        ammuksen_muoto,
                        ammuksen_sijainti.x,
                        ammuksen_sijainti.y,
                        Ammus,
                    ),
                    "ammus".to_string(),
                );

                // Lisätään ammukselle fysiikka ja ammuksen alkunopeus
                maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(
                    ammuksen_suunta * AMMUKSEN_NOPEUS,
                    r_kappale,
                ));
            }
        }
    }
}
