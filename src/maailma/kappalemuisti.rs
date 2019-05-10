use std::rc::Rc;

use super::kappale::Kappale;
use super::Lisaosa;
use super::RcKappale;

/// Muisti, joka muistaa kaksi kappaleen tilaa
pub struct Kappalemuisti {
    vanhin_versio: Kappale,
    uusin_versio: Kappale,
    oikea_versio: RcKappale,
}

impl Kappalemuisti {
    /// Luo uuden kappalemuistin tallentaen kopioiksi tämän hetkisen kappaleen tilan
    pub fn new(kappale: RcKappale) -> Self {
        Kappalemuisti {
            vanhin_versio: kopioi_sisalto(&kappale),
            uusin_versio: kopioi_sisalto(&kappale),
            oikea_versio: kappale,
        }
    }

    /// Päivitää kappaleen muistia, jolloin uusin versio tulee vanhimmaksi versioksi
    pub fn paivita_muistia(&mut self) {
        self.vanhin_versio = self.uusin_versio;
        self.uusin_versio = kopioi_sisalto(&self.oikea_versio);
    }

    pub fn aseta_tuleva_versio(&mut self, tuleva_versio: Kappale) {
        self.vanhin_versio = self.uusin_versio;
        self.uusin_versio = tuleva_versio;
    }

    /// Antaa kappaleen molemmat tallennetut versiot
    pub fn anna_versiot(&self) -> (&Kappale, &Kappale) {
        (&self.vanhin_versio, &self.uusin_versio)
    }
}

impl Lisaosa for Kappalemuisti {
    fn anna_kappale(&self) -> RcKappale {
        Rc::clone(&self.oikea_versio)
    }
}

/// Antaa kopion rckappaleen sisällöstä
fn kopioi_sisalto(rckappale: &RcKappale) -> Kappale {
    (*rckappale.borrow_mut()) // Kopioi sisällön
}
