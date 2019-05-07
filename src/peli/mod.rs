use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub mod pelihahmonpaivitys;
pub mod pelinpaivitys;

use crate::fysiikka::Fysiikkakappale;
use crate::maailma::kappale::Kappale;
use crate::maailma::vektori::Vektori;
use crate::maailma::*;
use crate::piirtaja::{PiirrettavaKappale, Piirtotapa};

/// Selkeyttää koodia, kun arvataan, että vektorilla tarkoitetaan luotavan kappaleen nopeutta ja suuntaa.
type Nopeus = Vektori;

/// Lisää kappaleen maailmaan, luoden sille piirrettävän lisäosan
/// # Arguments
/// * `maailma` - Pelimaailma, johon kappale lisätään
/// * `kappale` - Lisättävä kappale
/// * `vari` - Lisättävän kappaleen väri
pub fn lisaa_kappale(
    maailma: &mut Perusmaailma,
    kappale: Kappale,
    vari: Color,
) -> Rc<RefCell<Kappale>> {
    let r_kappale = maailma.lisaa_kappale(kappale);
    maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::new(
        Rc::clone(&r_kappale),
        Piirtotapa::Yksivarinen { vari: vari },
    ));
    r_kappale
}

/// Lisää kappaleen maailmaan, luoden sille piirrettävän lisäosan
/// # Arguments
/// * `maailma` - Pelimaailma, johon kappale lisätään
/// * `kappale` - Lisättävä kappale
/// * `vari` - Lisättävän kappaleen väri
pub fn lisaa_kuvallinen_kappale(
    maailma: &mut Perusmaailma,
    kappale: Kappale,
    kuva: String,
) -> Rc<RefCell<Kappale>> {
    let r_kappale = maailma.lisaa_kappale(kappale);
    maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::new(
        Rc::clone(&r_kappale),
        Piirtotapa::Kuvallinen { kuvan_nimi: kuva },
    ));
    r_kappale
}

/// Lisää fysiikkakappaleen kappaleineen maailmaan
/// /// # Arguments
/// * `maailma` - Pelimaailma, johon kappale lisätään
/// * `kappale` - Lisättävä kappale
/// * `vari` - Lisättävän kappaleen väri
pub fn lisaa_fysiikka_kappale(
    maailma: &mut Perusmaailma,
    kappale: Kappale,
    vari: Color,
) -> Rc<RefCell<Kappale>> {
    let r_kappale = maailma.lisaa_kappale(kappale);
    maailma.lisaa_piirrettava_kappale(PiirrettavaKappale::new(
        Rc::clone(&r_kappale),
        Piirtotapa::Yksivarinen { vari: vari },
    ));
    let f_kappale = Fysiikkakappale::new(Default::default(), Rc::clone(&r_kappale));
    maailma.lisaa_fysiikkakappale(f_kappale);
    r_kappale
}
