use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::fysiikka::Fysiikkakappale;
use crate::maailma::kappale::Kappale;
use crate::maailma::vektori::Vektori;
use crate::maailma::Perusmaailma;
use crate::piirtaja::{PiirrettavaKappale, Piirtotapa};
use crate::tekoaly::{Aly, Alyllinen};

type RcKappale = Rc<RefCell<Kappale>>;

pub struct Spawneri {
    spawnin_vali: Duration,
    aikaa_seuraavaan_spawniin: Duration,
    kappale: Kappale,
    piirtotapa: Piirtotapa,
    nopeus: Option<Vektori>,
    aly: Option<Box<Aly>>,
}

impl Spawneri {
    pub fn new(
        spawnin_vali: Duration,
        kappale: Kappale,
        piirtotapa: Piirtotapa,
        nopeus: Option<Vektori>,
        aly: Option<Box<Aly>>,
    ) -> Spawneri {
        Spawneri {
            spawnin_vali: spawnin_vali,
            aikaa_seuraavaan_spawniin: spawnin_vali,
            kappale: kappale,
            piirtotapa: piirtotapa,
            nopeus: nopeus,
            aly: aly,
        }
    }

    pub fn paivita_spawneria(&mut self, maailma: &mut Perusmaailma, paivitys_aika: &Duration) {
        if self.aikaa_seuraavaan_spawniin <= *paivitys_aika {
            // Spawnataan kappale
            self.spawnaa(maailma);
            let ylijaava_aika = *paivitys_aika - self.aikaa_seuraavaan_spawniin;
            self.aikaa_seuraavaan_spawniin = self.spawnin_vali;

            // Paivitetaan spawnerin aikaa ylijäävällä ajalla rekursiivisesti
            self.paivita_spawneria(maailma, &ylijaava_aika);
        } else {
            self.aikaa_seuraavaan_spawniin -= *paivitys_aika;
        }
    }

    pub fn spawnaa(&mut self, maailma: &mut Perusmaailma) {
        let _rk = maailma.lisaa_kappale(self.kappale);
        if let Some(nopeus) = self.nopeus {
            maailma.lisaa_fysiikkakappale(Fysiikkakappale::new(nopeus, Rc::clone(&_rk)));
        }
        maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::new(
            Rc::clone(&_rk),
            self.piirtotapa.clone(),
        ));

        // Se on clone, ei copy!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        if let Some(alylaatikko) = &self.aly {
            maailma.lisaa_aly(Alyllinen::new(_rk, alylaatikko.clone()));
        }
    }
}

impl Clone for Box<Aly> {
    fn clone(&self) -> Box<Aly> {
        self.box_clone()
    }
}
