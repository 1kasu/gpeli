use std::rc::Rc;

use super::vektori::Vektori;
use super::Lisaosa;
use super::RcKappale;

/// Pelissä oleva pelaajan ohjaama hahmo. Esimerkiksi kamera seuraa automaattisesti tätä.
pub struct Pelihahmo {
    /// Pelihahmon käyttämä kappale
    kappale: RcKappale,
    /// Pelihahmon viimeisin suunta esim. minne katsoo, ampuu jne
    suunta: Vektori<f32>,
}

impl Pelihahmo {
    // Lisää annetun kappaleen pelihahmoksi.
    pub fn new(kappale: RcKappale) -> Self {
        Pelihahmo {
            kappale: kappale,
            suunta: Vektori::new(1.0, 0.0).yksikkovektori(),
        }
    }

    /// Antaa pelihahmon suunnan
    pub fn anna_suunta(&self) -> Vektori<f32> {
        self.suunta
    }

    /// Asettaa uuden suunnan pelihahmolle
    /// # Arguments
    /// * `suunta` - Pelihahmon uusi suunta.
    pub fn aseta_suunta(&mut self, suunta: Vektori<f32>) {
        let yksikkovektori = suunta.yksikkovektori();
        if yksikkovektori.x.is_finite() && yksikkovektori.y.is_finite() {
            self.suunta = yksikkovektori;
        }
    }
}

impl Lisaosa for Pelihahmo {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.kappale)
    }
}
